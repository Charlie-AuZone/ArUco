export DEBIAN_FRONTEND=noninteractive

apt update
apt install git wget gpg clang libclang-dev zip unzip curl -y

wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null | gpg --dearmor - | tee /usr/share/keyrings/kitware-archive-keyring.gpg >/dev/null
echo 'deb [signed-by=/usr/share/keyrings/kitware-archive-keyring.gpg] https://apt.kitware.com/ubuntu/ focal-rc main' | tee -a /etc/apt/sources.list.d/kitware.list >/dev/null
apt update
apt install cmake

wget -O opencv.zip https://github.com/opencv/opencv/archive/refs/tags/4.8.1.zip
wget -O opencv_contrib.zip https://github.com/opencv/opencv_contrib/archive/refs/tags/4.8.1.zip
unzip opencv.zip && rm opencv.zip
unzip opencv_contrib.zip && rm opencv_contrib.zip
mkdir -p build && cd build
cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=NO -DCMAKE_INSTALL_PREFIX=/opt/opencv -DBUILD_DOCS=OFF -DBUILD_EXAMPLES=OFF -DBUILD_TESTS=OFF -DBUILD_PERF_TESTS=OFF -DWITH_PNG=OFF -DWITH_JPEG=OFF -DWITH_TIFF=OFF -DWITH_WEBP=OFF -DWITH_OPENJPEG=OFF -DWITH_JASPER=OFF -DWITH_OPENEXR=OFF -DWITH_V4L=OFF  -DBUILD_opencv_java=OFF -DBUILD_opencv_python=OFF -DOPENCV_EXTRA_MODULES_PATH=../opencv_contrib-4.8.1/modules ../opencv-4.8.1
cmake --build . --target install --config Release --parallel 8
cmake --install . --prefix /opt/opencv
cd ..

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

git clone https://github.com/Charlie-AuZone/ArUco.git
cd ArUco
~/.cargo/bin/cargo build
