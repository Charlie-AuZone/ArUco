FROM ubuntu:20.04
WORKDIR /root/

RUN apt update && DEBIAN_FRONTEND=noninteractive apt install git wget gpg clang libclang-dev zip unzip curl -y

RUN wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null | gpg --dearmor - | tee /usr/share/keyrings/kitware-archive-keyring.gpg >/dev/null
RUN echo 'deb [signed-by=/usr/share/keyrings/kitware-archive-keyring.gpg] https://apt.kitware.com/ubuntu/ focal-rc main' | tee -a /etc/apt/sources.list.d/kitware.list >/dev/null
RUN apt update
RUN DEBIAN_FRONTEND=noninteractive apt install cmake

RUN wget -O opencv.zip https://github.com/opencv/opencv/archive/refs/tags/4.8.1.zip
RUN wget -O opencv_contrib.zip https://github.com/opencv/opencv_contrib/archive/refs/tags/4.8.1.zip
RUN unzip opencv.zip && rm opencv.zip
RUN unzip opencv_contrib.zip && rm opencv_contrib.zip
RUN mkdir -p build && cd build
RUN cmake -DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=NO -DCMAKE_INSTALL_PREFIX=/opt/opencv -DBUILD_DOCS=OFF -DBUILD_EXAMPLES=OFF -DBUILD_TESTS=OFF -DBUILD_PERF_TESTS=OFF -DWITH_PNG=OFF -DWITH_JPEG=OFF -DWITH_TIFF=OFF -DWITH_WEBP=OFF -DWITH_OPENJPEG=OFF -DWITH_JASPER=OFF -DWITH_OPENEXR=OFF -DWITH_V4L=OFF  -DBUILD_opencv_java=OFF -DBUILD_opencv_python=OFF -DOPENCV_EXTRA_MODULES_PATH=../opencv_contrib-4.8.1/modules ../opencv-4.8.1
RUN cmake --build . --target install --config Release --parallel 8
RUN cmake --install . --prefix /opt/opencv
RUN cd ..

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN git clone https://github.com/Charlie-AuZone/ArUco.git
RUN cd ArUco
RUN ~/.cargo/bin/cargo build
COPY target/release/trimble-aruco-bench /