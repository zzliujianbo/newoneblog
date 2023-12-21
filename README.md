# newoneblog
newoneblog是一个基于rust开发的博客程序，实现了类似于`github pages`的功能。

主要功能：

1、自动将markdown目录转义为html静态文件。

2、静态文件模板功能。

3、web服务。

newoneblog主要基于poem、xx开发。

依赖内容

# 1、使用方法
## 1.1、安装
``` sh
curl xxx
```
## 1.2、创建配置文件
``` json
{
    //绑定的服务器ip
    "server_ip": "",
    //绑定的服务器端口
    "server_port": 8080,
    //title
    "title": "newoneblog",
    //keyword
    "keyword": "博客|blog",
    //markdown目录
    "markdown_dir": "./markdown",
    //统计脚本（考虑脚本方式）
    //模板路径
    "template_path": "./template",
    //public目录
    "public_path": "./public",
    //关于我们
    "about": "newoneblog是一个基于rust开发的博客程序，实现了类似于github pages的功能。"
}

```
## 1.3、启动

运行后会根据配置的markdown目录，自动解析目录下的所有md文件。并缓存到public目录下。
public目录为主要对外提供可访问目录。


## 1.4、重新缓存
当md文件有新的更新后，调用xx接口并加上密钥重新生成缓存。

# 2、开发
``` rust
cargo build
```
编译完成之后