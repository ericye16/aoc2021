package(default_visibility = ["//visibility:public"])

[filegroup(
    name = f[:-4],  # strip .txt
    srcs = [f],
) for f in glob(["d*.txt"])]
