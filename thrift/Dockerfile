FROM fedora:27

RUN dnf -y groupinstall "Development Tools"
RUN dnf -y install \
      bison \
      bison-devel \
      flex \
      flex-devel \
      gcc-c++ \
      git \
      libtool \
      qt-devel \
      wget

RUN wget http://archive.apache.org/dist/thrift/0.11.0/thrift-0.11.0.tar.gz \
  && mkdir /thrift \
  && tar --extract --strip 1 --file thrift-0.11.0.tar.gz --directory /thrift

WORKDIR /thrift
RUN ./bootstrap.sh \
  && ./configure --with-lua=no --with-rust=yes \
  && make \
  && make install
