#include <jni.h>
#include <string>
#include <android/log.h>

// Native function declarations from Rust
extern "C" {
    const char* create_post_native(const char* post_json);
    const char* get_timeline_native(const char* user_id, int limit, int offset);
    const char* get_post_native(const char* post_id);
    const char* create_relationship_native(const char* follower_id, const char* followed_id);
    const char* get_followers_native(const char* user_id);
    const char* get_following_native(const char* user_id);
}

// Logging tag
#define LOG_TAG "CPC_CORE"

// Helper function for logging
void log_message(const char* message) {
    __android_log_print(ANDROID_LOG_INFO, LOG_TAG, "%s", message);
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_cpc_social_ffi_SocialNative_createPostNative(JNIEnv* env, jobject /* this */, jstring post_json) {
    const char* post_json_str = env->GetStringUTFChars(post_json, nullptr);
    log_message("Creating post from JSON");
    
    const char* result = create_post_native(post_json_str);
    
    env->ReleaseStringUTFChars(post_json, post_json_str);
    return env->NewStringUTF(result);
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_cpc_social_ffi_SocialNative_getTimelineNative(JNIEnv* env, jobject /* this */, jstring user_id, jint limit, jint offset) {
    const char* user_id_str = env->GetStringUTFChars(user_id, nullptr);
    log_message("Getting timeline for user");
    
    const char* result = get_timeline_native(user_id_str, limit, offset);
    
    env->ReleaseStringUTFChars(user_id, user_id_str);
    return env->NewStringUTF(result);
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_cpc_social_ffi_SocialNative_getPostNative(JNIEnv* env, jobject /* this */, jstring post_id) {
    const char* post_id_str = env->GetStringUTFChars(post_id, nullptr);
    log_message("Getting post by ID");
    
    const char* result = get_post_native(post_id_str);
    
    env->ReleaseStringUTFChars(post_id, post_id_str);
    return env->NewStringUTF(result);
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_cpc_social_ffi_SocialNative_createRelationshipNative(JNIEnv* env, jobject /* this */, jstring follower_id, jstring followed_id) {
    const char* follower_id_str = env->GetStringUTFChars(follower_id, nullptr);
    const char* followed_id_str = env->GetStringUTFChars(followed_id, nullptr);
    log_message("Creating relationship");
    
    const char* result = create_relationship_native(follower_id_str, followed_id_str);
    
    env->ReleaseStringUTFChars(follower_id, follower_id_str);
    env->ReleaseStringUTFChars(followed_id, followed_id_str);
    return env->NewStringUTF(result);
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_cpc_social_ffi_SocialNative_getFollowersNative(JNIEnv* env, jobject /* this */, jstring user_id) {
    const char* user_id_str = env->GetStringUTFChars(user_id, nullptr);
    log_message("Getting followers for user");
    
    const char* result = get_followers_native(user_id_str);
    
    env->ReleaseStringUTFChars(user_id, user_id_str);
    return env->NewStringUTF(result);
}

extern "C"
JNIEXPORT jstring JNICALL
Java_com_cpc_social_ffi_SocialNative_getFollowingNative(JNIEnv* env, jobject /* this */, jstring user_id) {
    const char* user_id_str = env->GetStringUTFChars(user_id, nullptr);
    log_message("Getting following for user");
    
    const char* result = get_following_native(user_id_str);
    
    env->ReleaseStringUTFChars(user_id, user_id_str);
    return env->NewStringUTF(result);
}