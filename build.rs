fn main() {
    cc::Build::new()
        .cpp(true)
        .define("HAVE_CONFIG_H", "1")
        .file("encoder.cpp")
        .file("feature.cpp")
        .file("feature_cache.cpp")
        .file("libcrfpp.cpp")
        .file("feature_index.cpp")
        .file("node.cpp")
        .file("param.cpp")
        .file("path.cpp")
        .file("tagger.cpp")
        .file("lbfgs.cpp")
        .compile("crf");
}
