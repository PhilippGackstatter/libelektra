FROM fedora:32

RUN dnf upgrade -y && dnf install -y \
        augeas-devel \
        boost-devel \
        cmake \
        curl \
        dbus-devel \
        diffutils \
        file \
        findutils \
        gcc-c++ \
        git \
        glib2 \
        gpgme-devel \
        java-1.8.0-openjdk-devel \
        jna \
        libcurl-devel \
        libev-devel \
        libgcrypt-devel \
        libgit2-devel \
        libmarkdown-devel \
        libuv-devel \
        libxml2-devel \
        lua-devel \
        make \
        maven \
        ninja-build \
        openssl-devel \
        procps-ng \
        python3-devel \
        qt5-devel \
        ruby-devel \
        rubygem-test-unit \
        swig \
        valgrind \
        xerces-c-devel \
        yajl-devel \
        yaml-cpp-devel \
        zlib-devel \
    && dnf clean all -y

# Google Test
ENV GTEST_ROOT=/opt/gtest
ARG GTEST_VER=release-1.10.0
RUN mkdir -p ${GTEST_ROOT} \
    && cd /tmp \
    && curl -o gtest.tar.gz \
      -L https://github.com/google/googletest/archive/${GTEST_VER}.tar.gz \
    && tar -zxvf gtest.tar.gz --strip-components=1 -C ${GTEST_ROOT} \
    && rm gtest.tar.gz

