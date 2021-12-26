load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name = "p1",
    srcs = ["p1.cpp"],
    data = ["//data:d1"],
    deps = [
        "@bazel_tools//tools/cpp/runfiles",
        "@com_github_google_glog//:glog",
    ],
)
