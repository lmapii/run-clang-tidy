
@REM powershell -Command "Invoke-WebRequest https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.0/LLVM-14.0.0-win64.exe -OutFile llvm.exe"
@REM choco install python3 -y
@REM choco install choco install mingw -y

choco install make

pip3 install argparse datetime

choco install llvm --version 14.0.0 -y
dir "%SYSTEMDRIVE%\Program Files\LLVM\bin"

mkdir "artifacts\clang"
copy "%SYSTEMDRIVE%\Program Files\LLVM\bin\clang-tidy.exe" "artifacts\clang"
copy "%SYSTEMDRIVE%\Program Files\LLVM\bin\clang.exe" "artifacts\clang"

make -C test-files\c-demo\project build-data
dir test-files\c-demo\_bld\out

make -C test-files\c-demo\project
