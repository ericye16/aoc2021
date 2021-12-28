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

cc_binary(
    name = "p22",
    srcs = ["p22.cpp"],
    data = [
        "//data:d22",
        "//data:d22-example",
        "//data:d22-example2",
        "//data:d22-example3",
    ],
    deps = [
        "@bazel_tools//tools/cpp/runfiles",
        "@com_github_google_glog//:glog",
        "@com_google_absl//absl/flags:flag",
        "@com_google_absl//absl/flags:parse",
    ],
)

cc_binary(
    name = "p23",
    srcs = ["p23.cpp"],
    data = [
        "//data:d23",
        "//data:d23-done",
        "//data:d23-example1",
    ],
    deps = [
        "@bazel_tools//tools/cpp/runfiles",
        "@com_github_google_glog//:glog",
        "@com_google_absl//absl/flags:flag",
        "@com_google_absl//absl/flags:parse",
        "@com_google_absl//absl/container:flat_hash_set",
    ],
)
