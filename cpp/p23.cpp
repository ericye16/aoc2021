#include <fstream>
#include <iostream>
#include <queue>
#include <string>
#include <unordered_set>

#include "absl/container/flat_hash_set.h"
#include "absl/flags/flag.h"
#include "absl/flags/parse.h"
#include "glog/logging.h"
#include "tools/cpp/runfiles/runfiles.h"

ABSL_FLAG(std::string, data_file, "d23.txt", "Data file");

using bazel::tools::cpp::runfiles::Runfiles;

using MoveQueue =
    std::priority_queue<std::pair<uint64_t, std::string>,
                        std::vector<std::pair<uint64_t, std::string>>,
                        std::greater<std::pair<uint64_t, std::string>>>;

constexpr size_t COLSIZE = 13;
constexpr size_t HALLWAY_START = 1;
constexpr size_t HALLWAY_STOP = 12;

constexpr size_t HALLWAY_ROW = 1;


size_t animal_to_room(char c) {
  if (c == 'A')
    return 3;
  else if (c == 'B')
    return 5;
  else if (c == 'C')
    return 7;
  else {
    DCHECK(c == 'D');
    return 9;
  }
}

std::string parse_input(std::ifstream&& infile) {
  CHECK(infile.is_open());
  std::stringstream ss;
  for (std::string line; std::getline(infile, line);) {
    ss << line;
    if (line.size() < COLSIZE) {
      ss << "  ";
    }
  }

  return ss.str();
}

inline size_t rc_to_index(size_t row, size_t col) { return row * COLSIZE + col; }

inline std::pair<size_t, size_t> index_to_rc(size_t index) {
  return {index / COLSIZE, index % COLSIZE};
}

inline std::optional<size_t> get_unoccupied_position(char animal,
                                              std::string_view grid,
                                              size_t num_animals) {
  size_t col = animal_to_room(animal);
  for (size_t row = HALLWAY_ROW + num_animals; row > HALLWAY_ROW; row--) {
    char occupant = grid[rc_to_index(row, col)];
    DCHECK(occupant != '#');
    if (occupant != '.' && occupant != animal) {
      return std::nullopt;
    } else if (occupant == '.') {
      return row;
    }
  }
  return HALLWAY_ROW;
}

inline bool is_done(std::string_view grid, size_t room_size) {
  // Check hallway is clear
  /*
  for (size_t col = HALLWAY_START; col < HALLWAY_STOP; col++) {
    if (grid[rc_to_index(HALLWAY_ROW, col)] != '.') {
      return false;
    }
  }
  */
  for (size_t row = HALLWAY_ROW + 1; row < HALLWAY_ROW + 1 + room_size; row++) {
    for (char animal = 'A'; animal <= 'D'; animal++) {
      auto unoccupied = get_unoccupied_position(animal, grid, room_size);
      if (!unoccupied.has_value() || *unoccupied != HALLWAY_ROW) {
        return false;
      }
    }
  }
  return true;
}

// * inclusive on a, exclusive on b * //
inline bool hallway_clear_between(std::string_view grid, size_t a, size_t b) {
  size_t lo;
  size_t hi;
  if (a > b) {
    lo = b + 1;
    hi = a + 1;
  } else {
    lo = a;
    hi = b;
  }
  for (size_t i = lo; i < hi; i++) {
    if (grid[rc_to_index(HALLWAY_ROW, i)] != '.') {
      return false;
    }
  }
  return true;
}

inline uint64_t energy_for(char animal) {
  if (animal == 'A') {
    return 1;
  } else if (animal == 'B') {
    return 10;
  } else if (animal == 'C') {
    return 100;
  } else {
    DCHECK(animal == 'D');
    return 1000;
  }
}

inline bool above_room(size_t col) {
  return col == 3 || col == 5 || col == 7 || col == 9;
}

inline void maybe_insert_new_grid(std::pair<size_t, size_t> target_coords,
                           std::pair<size_t, size_t> current_coords,
                           uint64_t current_energy,
                           std::string_view current_grid,
                           const absl::flat_hash_set<std::string> visited,
                           MoveQueue* queue) {
  size_t steps = size_t(
      std::abs(int32_t(target_coords.first) - int32_t(current_coords.first)) +
      std::abs(int32_t(target_coords.second) - int32_t(current_coords.second)));
  char animal =
      current_grid[rc_to_index(current_coords.first, current_coords.second)];
  uint64_t new_energy = current_energy + steps * energy_for(animal);
  std::string new_grid(current_grid);
  DCHECK(new_grid[rc_to_index(target_coords.first, target_coords.second)] ==
         '.');
  DCHECK(new_grid[rc_to_index(current_coords.first, current_coords.second)] ==
         animal);
  new_grid[rc_to_index(target_coords.first, target_coords.second)] = animal;
  new_grid[rc_to_index(current_coords.first, current_coords.second)] = '.';
  if (visited.find(new_grid) == visited.end()) {
    queue->emplace(new_energy, std::move(new_grid));
  }
}

std::string pretty_grid(std::string_view grid) {
  std::stringstream ss;
  size_t idx = 0;
  while (idx + COLSIZE <= grid.size()) {
    ss << grid.substr(idx, COLSIZE) << '\n';
    idx += COLSIZE;
  }
  return ss.str();
}

uint64_t p1(std::string input) {
  const size_t num_animals = 2;
  auto queue = MoveQueue();
  queue.emplace(0, input);
  auto visited = absl::flat_hash_set<std::string>();
  while (!queue.empty()) {
    const auto [energy, grid] = queue.top();
    queue.pop();
    DCHECK(grid.size() == COLSIZE * 5);
    if (!visited.insert(grid).second) {
      continue;
    }
    if (visited.size() % 10000 == 0) {
      std::cout << "Energy: " << energy << ", to explore: " << queue.size()
                << ", visited: " << visited.size() << " grid:\n"
                << pretty_grid(grid) << std::endl;
    }
    visited.insert(grid);
    if (is_done(grid, num_animals)) {
      std::cout << "Energy: " << energy << ", to explore: " << queue.size()
                << ", visited: " << visited.size() << " grid:\n"
                << pretty_grid(grid) << std::endl;
      return energy;
    }
    // Look in hallway
    for (size_t col = HALLWAY_START; col < HALLWAY_STOP; col++) {
      char occupant = grid[rc_to_index(HALLWAY_ROW, col)];
      if (occupant == '.') {
        continue;
      }
      size_t target_col = animal_to_room(occupant);
      bool hallway_clear = hallway_clear_between(grid, target_col, col);
      auto maybe_seat = get_unoccupied_position(occupant, grid, num_animals);
      if (hallway_clear && maybe_seat.has_value()) {
        DCHECK(*maybe_seat > HALLWAY_ROW);
        maybe_insert_new_grid({*maybe_seat, target_col}, {HALLWAY_ROW, col},
                              energy, grid, visited, &queue);
      }
    }
    // Check rooms
    for (char animal = 'A'; animal <= 'D'; animal++) {
      if (get_unoccupied_position(animal, grid, num_animals).has_value()) {
        continue;
      }
      size_t target_col = animal_to_room(animal);
      for (size_t row = HALLWAY_ROW + 1; row < HALLWAY_ROW + 1 + num_animals;
           row++) {
        if (grid[rc_to_index(row, target_col)] == '.') {
          continue;
        }
        // At this point, (row, target_col) contains the animal we want to move
        size_t col = target_col - 1;
        while (grid[rc_to_index(HALLWAY_ROW, col)] == '.') {
          if (above_room(col)) {
            col--;
            continue;
          }
          maybe_insert_new_grid({HALLWAY_ROW, col}, {row, target_col}, energy,
                                grid, visited, &queue);
          col--;
        }
        col = target_col + 1;
        while (grid[rc_to_index(HALLWAY_ROW, col)] == '.') {
          if (above_room(col)) {
            col++;
            continue;
          }
          maybe_insert_new_grid({HALLWAY_ROW, col}, {row, target_col}, energy,
                                grid, visited, &queue);
          col++;
        }
        break;
      }
    }
  }
  return UINT64_MAX;
}

int main(int argc, char** argv) {
  google::InitGoogleLogging(argv[0]);
  google::InstallFailureSignalHandler();
  absl::ParseCommandLine(argc, argv);
  std::string error;
  std::unique_ptr<Runfiles> runfiles(Runfiles::Create(argv[0], &error));

  if (runfiles == nullptr) {
    LOG(ERROR) << "Couldn't open runfiles";
    return -1;
  }

  std::string path =
      runfiles->Rlocation("__main__/data/" + absl::GetFlag(FLAGS_data_file));
  const auto v = parse_input(std::ifstream(path));
  std::cout << "input:\n" << pretty_grid(v) << std::endl;
  CHECK(v.size() == 13 * 5);

  std::cout << "P1: " << p1(v) << std::endl;
  // std::cout << "P2: " << p2(v) << std::endl;

  return 0;
}
