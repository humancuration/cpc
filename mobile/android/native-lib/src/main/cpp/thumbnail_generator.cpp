#include <jni.h>
#include <string>

extern "C" {
    #include "cpc_core.h"
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_cpcstudio_thumbnail_ThumbnailGenerator_generateThumbnail(
        JNIEnv* env,
        jobject /* this */,
        jstring modelPath,
        jstring outputPath,
        jint size) {
    const char *model_path = env->GetStringUTFChars(modelPath, nullptr);
    const char *output_path = env->GetStringUTFChars(outputPath, nullptr);
    
    // Call Rust function
    const char* error = generate_model_thumbnail(model_path, output_path, static_cast<unsigned int>(size));
    
    env->ReleaseStringUTFChars(modelPath, model_path);
    env->ReleaseStringUTFChars(outputPath, output_path);
    
    if (error != nullptr) {
        return env->NewStringUTF(error);
    }
    
    return nullptr;
}