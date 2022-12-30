# libgdbm-rust

#### Description
rewrite gdbm tool by using Rust

#### Software Architecture
GDBM - GNU database manager，是一套简单的资料库管理函数。dbm 是UNIX 上一套古老的常见资料库处理介面，而ndbm 则是改良dbm 缺失而设计的，两者的介面差异颇大。gdbm 则是根据ndbm 的介面设计的，两者间非常相似， ndbm 的函数名称皆以dbm_ 开头，而gdbm 为了加以区别，则以gdbm_ 开头，除此之外，皆可对应。gdbm 所提供的资料库管理介面，与ndbm 非常相似，因此gdbm 的说明，也可以适用于ndbm 上。
原始的gdbm是由c语言编写完成的，由于c语言的天生问题，gdbm存在很多隐性的内存方面的bug。本项目中使用rust语言基于gdbm-1.23版本改写gdbm代码，从而消除内存方面的隐患
#### Installation

1.  sh build.sh

#### Contribution

1.  Fork the repository
2.  Create Feat_xxx branch
3.  Commit your code
4.  Create Pull Request


#### Gitee Feature

1.  You can use Readme\_XXX.md to support different languages, such as Readme\_en.md, Readme\_zh.md
2.  Gitee blog [blog.gitee.com](https://blog.gitee.com)
3.  Explore open source project [https://gitee.com/explore](https://gitee.com/explore)
4.  The most valuable open source project [GVP](https://gitee.com/gvp)
5.  The manual of Gitee [https://gitee.com/help](https://gitee.com/help)
6.  The most popular members  [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)
