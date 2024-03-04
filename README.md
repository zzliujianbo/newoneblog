# newoneblog

newoneblog 是一个博客程序，将 markdown 文档转换为 html 文件。并提供 web 静态文件服务。

## 1、创建配置文件

```json
{
    "server_ip": "127.0.0.1",
    "server_port": 3000,
    "title": "newoneblog",
    "keywords": "rust, blog, newoneblog, markdown",
    "description": "一个通过markdown生成博客的程序",
    "markdown_path": "./markdown",
    "template_path": "./templates/newoneblog",
    "public_path": "./public",
    "ignore_markdown_path": [
        "./markdown/.git",
        "./markdown/ignore-markdown"
    ],
    "footer_html": "<div>xx备案</div>",
    "global_include_script": [
        "./include_script/test.js"
    ],
    "global_include_script_path": [
        "https://cdnjs.cloudflare.com/ajax/libs/jquery/3.7.1/jquery.min.js"
    ]
}
```

| 配置字段                   | 描述                                                                        |
| -------------------------- | --------------------------------------------------------------------------- |
| server_ip                  | web 服务 ip 地址                                                            |
| server_port                | web 服务端口                                                                |
| title                      | 网站标题                                                                    |
| keyword                    | 网站关键字                                                                  |
| markdown_dir               | markdown 目录                                                               |
| template_path              | 网站模板目录                                                                |
| public_path                | 生成的 html 存放目录                                                        |
| ignore_markdown_path       | 忽略的 markdown 文件或者路径，需要以markdown_dir路径开头                    |
| footer_html                | 底部html代码                                                                |
| global_include_script      | 全局运行的script脚本，在base.html页面中以`<script>code</script>`方式 引入   |
| global_include_script_path | 全局引入的script脚本，在base.html页面中以`<script src=""></script>`方式引入 |
## 2、启动

```shell
cargo run
```
或者
```
cargo watch -x "run --bin newoneblog"
```
运行后会根据 markdown 和 template 目录，生成 html 文件到 public 目录下。

访问 `http://127.0.0.1:3000` 查看网站。

![网站图片](website.png)
![网站图片](website2.png)

## 3、编译

交叉编译linux：
``` shell
cargo build --release --target=x86_64-unknown-linux-musl -vv
```