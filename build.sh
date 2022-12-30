cargo build;
path=`pwd`
ln -s $path/target/debug/libgdbm_rust.so $path/src/.libs/libgdbm.so
make

if [ "$1" == "clean" ];then
rm -rf $path/src/.libs/libgdbm.so
make clean
fi
