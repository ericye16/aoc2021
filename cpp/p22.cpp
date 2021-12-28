#include <fstream>
#include <iostream>
#include <numeric>
#include <optional>
#include <regex>
#include <string>

#include "absl/flags/flag.h"
#include "absl/flags/parse.h"
#include "glog/logging.h"
#include "tools/cpp/runfiles/runfiles.h"

ABSL_FLAG(std::string, data_file, "d22.txt", "File to read");

using bazel::tools::cpp::runfiles::Runfiles;

struct Cube {
  int xmin, xmax;
  int ymin, ymax;
  int zmin, zmax;
};

std::vector<std::pair<Cube, bool>> parse_input(std::ifstream&& ifile) {
  CHECK(ifile.is_open());
  std::regex line(
      R"##(([a-z]+) x=([\d\-]+)..([\d\-]+),y=([\d\-]+)..([\d\-]+),z=([\d\-]+)..([\d\-]+))##");
  auto v = std::vector<std::pair<Cube, bool>>();
  for (std::string s; std::getline(ifile, s);) {
    std::smatch matches;
    CHECK(std::regex_match(s, matches, line));

    v.emplace_back(
        Cube{
            .xmin = std::stoi(matches[2]),
            .xmax = std::stoi(matches[3]),
            .ymin = std::stoi(matches[4]),
            .ymax = std::stoi(matches[5]),
            .zmin = std::stoi(matches[6]),
            .zmax = std::stoi(matches[7]),
        },
        matches[1] == "on");
  }
  return v;
}

std::optional<Cube> get_intersection(const Cube& a, const Cube& b) {
  auto cube = Cube{
      .xmin = std::max(a.xmin, b.xmin),
      .xmax = std::min(a.xmax, b.xmax),
      .ymin = std::max(a.ymin, b.ymin),
      .ymax = std::min(a.ymax, b.ymax),
      .zmin = std::max(a.zmin, b.zmin),
      .zmax = std::min(a.zmax, b.zmax),
  };
  if (cube.xmin > cube.xmax || cube.ymin > cube.ymax || cube.zmin > cube.zmax) {
    return std::nullopt;
  }
  return cube;
}

std::optional<Cube> clip_cube(const Cube& cube) {
  return get_intersection(
      Cube{
          .xmin = -50,
          .xmax = 50,
          .ymin = -50,
          .ymax = 50,
          .zmin = -50,
          .zmax = 50,
      },
      cube);
}

int p1(const std::vector<std::pair<Cube, bool>>& steps) {
  auto cubes = std::vector<std::pair<Cube, int>>();
  for (const auto& [cube, on] : steps) {
    const auto cube_clipped = clip_cube(cube);
    if (!cube_clipped.has_value()) {
      continue;
    }
    auto intersecting = std::vector<std::pair<Cube, int>>();
    for (const auto& [existing_cube, weight] : cubes) {
      if (auto intersection = get_intersection(existing_cube, *cube_clipped)) {
        intersecting.emplace_back(*intersection, weight * -1);
      }
    }
    if (on) {
      intersecting.emplace_back(*cube_clipped, 1);
    }
    cubes.insert(cubes.end(), intersecting.begin(), intersecting.end());
  }
  int on = 0;
  for (const auto& [cube, weight] : cubes) {
    on += (cube.xmax - cube.xmin + 1) * (cube.ymax - cube.ymin + 1) *
          (cube.zmax - cube.zmin + 1) * weight;
  }
  return on;
}

int64_t p2(const std::vector<std::pair<Cube, bool>>& steps) {
  auto cubes = std::vector<std::pair<Cube, int>>();
  for (const auto& [cube, on] : steps) {
    auto intersecting = std::vector<std::pair<Cube, int>>();
    for (const auto& [existing_cube, weight] : cubes) {
      if (auto intersection = get_intersection(existing_cube, cube)) {
        intersecting.emplace_back(*intersection, weight * -1);
      }
    }
    if (on) {
      intersecting.emplace_back(cube, 1);
    }
    cubes.insert(cubes.end(), intersecting.begin(), intersecting.end());
  }
  int64_t on = 0;
  for (const auto& [cube, weight] : cubes) {
    on += int64_t(cube.xmax - cube.xmin + 1) *
          int64_t(cube.ymax - cube.ymin + 1) *
          int64_t(cube.zmax - cube.zmin + 1) * weight;
  }
  return on;
}

int main(int argc, char** argv) {
  google::InitGoogleLogging(argv[0]);
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

  std::cout << "P1: " << p1(v) << std::endl;
  std::cout << "P2: " << p2(v) << std::endl;

  return 0;
}
