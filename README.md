# Basic Warp API

diesel cli
warp


Heroku deploy

Procfile->
web ./target/release/family-api


RustConfig->
VERSION=nightly


heroku buildpacks->
heroku create --buildpack emk/rust
heroku buildpacks:set emk/rust

Random String using node
require('crypto').randomBytes(64).toString('hex')

Apk Deploy
work="ionic build --prod"
eval $work
printf "Build finished.\nNow starting sync!\n"
sync="ionic cap sync"
eval $sync
printf "Sync finished.\nNow coping native android files!\n"
copy="ionic cap copy android"
eval $copy
printf "Copy finished.\nStarting apk build!\n"
apk="cd android && ./gradlew assembleDebug"
eval $apk
copy_apk="cd .. && cp android/app/build/outputs/apk/debug/app-debug.apk APK && cp android/app/build/outputs/apk/debug/output-metadata.json APK"
eval $copy_apk
