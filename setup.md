git clone https://github.com/microsoft/vcpkg
c:\vcpkg\bootstrap-vcpkg.bat
vcpkg install --recurse curl[core,sspi,http2,non-http,ssl]:x64-windows-static-md
vcpkg install --recurse libxml2[core,iconv]:x64-windows-static-md

vcpkg integrate install

(Add path to user as well!)