#include <ctime>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>

#include "glog/logging.h"
#include "tools/cpp/runfiles/runfiles.h"

using bazel::tools::cpp::runfiles::Runfiles;

int p1(std::ifstream&& ifile) {
  CHECK(ifile.is_open());
  int last;
  ifile >> last;
  int a;
  int increments = 0;
  while (ifile >> a) {
    if (a > last) {
      increments++;
    }
    last = a;
  }
  return increments;
}

int p2(std::ifstream&& ifile) {
  CHECK(ifile.is_open());
  std::deque<int> window = {};
  int a;
  for (int i = 0; i < 3; i++) {
    ifile >> a;
    window.push_back(a);
  }
  int increments = 0;
  int last = std::accumulate(window.begin(), window.end(), 0);
  while (ifile >> a) {
    window.push_back(a);
    window.pop_front();
    int new_sum = std::accumulate(window.begin(), window.end(), 0);
    if (new_sum > last) {
      increments++;
    }
    last = new_sum;
  }
  return increments;
}

int main(int argc, char** argv) {
  google::InitGoogleLogging(argv[0]);
  std::string error;
  std::unique_ptr<Runfiles> runfiles(Runfiles::Create(argv[0], &error));

  if (runfiles == nullptr) {
    LOG(ERROR) << "Couldn't open runfiles";
    return -1;
  }

  std::string path = runfiles->Rlocation("__main__/data/d1.txt");

  std::cout << "P1: " << p1(std::ifstream(path)) << std::endl;
  std::cout << "P2: " << p2(std::ifstream(path)) << std::endl;

  return 0;
}
