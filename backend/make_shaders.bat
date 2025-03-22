@echo off

cls
echo + build +

set arg=%1
if "%arg%"=="" set arg=run

sokol-shdc.exe -i ./src/shaders.glsl -o ./src/shaders.rs --slang hlsl5:wgsl:glsl430 -f sokol_rust

cargo %arg%
