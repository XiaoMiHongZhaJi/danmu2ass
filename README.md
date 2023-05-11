# danmu2ass
[![Continuous integration](https://github.com/gwy15/danmu2ass/actions/workflows/ci.yml/badge.svg)](https://github.com/gwy15/danmu2ass/actions/workflows/ci.yml)
[![Publish Docker image](https://github.com/gwy15/danmu2ass/actions/workflows/docker.yml/badge.svg)](https://github.com/gwy15/danmu2ass/actions/workflows/docker.yml)

将哔哩哔哩的xml弹幕文件转化为ass格式

## 支持的格式
- [x] 哔哩哔哩直播：录播姬等录制的 XML 格式文件
- [x] 哔哩哔哩视频：支持 BV 号/链接（可带分 p 参数）


## 特性
- 比 danmaku2ass 快一百倍的速度（见下方性能对比）
- 更紧密的弹幕填充算法（见下）
- 底部和顶部弹幕和逆向弹幕转成正常弹幕，减少遮挡
- 弹幕透明度、字体、字号、高度、间距等全部可调
- 支持过滤黑名单关键词
- 支持文件夹模式，递归查找所有 xml 文件并多线程处理
- 自动判断是否已经转换过，跳过已转换的文件，方便自动化处理
- 编译为二进制，支持 docker 部署，不需要 python 环境

![填充算法示例](./resources/sample.png)

## 性能对比
xml 解析器默认使用 quick_xml

测试 238M 的文件：
- `quick-xml`：449.5ms 461.3ms 505.1ms 406.23ms
- `xml-rs`：18.0s 18.8s 18.2s 18.5s
- `danmaku2ass`：40.2s 40.8s 40.1s

# 安装
- 下载 https://github.com/XiaoMiHongZhaJi/danmu2ass 中的 release
    - 对于 Windows 用户，默认 zip 内会有一个 配置文件.toml，更改其中内容即可更改配置。
    - 该配置文件存在时不会解析命令行输入

# 使用
## 配置文件
当前目录下存在配置文件（文件名：配置文件.toml、可以用记事本编辑）时，
danmu2ass 会优先加载该配置文件而忽略任何命令行输入。
这也是 **推荐给不熟悉命令行的用户的方法** 。

> Windows 压缩包内默认包含配置文件。

编辑、保存完配置文件后双击 exe 即可转换，如果闪退请检查配置文件是否有格式错误。

```toml
# 在本文件存在时，优先读取本文件的内容，不会读取命令行参数

# 需要转换的输入，可以是 xml 文件、文件夹或是哔哩哔哩链接、BV 号。
# 如果是文件夹会递归将其下所有 XML 都进行转换
input = "."

# 输出的 ASS 文件，默认为输入文件名将 .xml 替换为 .ass，如果输入是文件夹则忽略
# ass_file = "out.ass"

# 弹幕时间轴偏移，>0 会让弹幕延后，<0 会让弹幕提前，单位为秒
time_offset = 0.0

# 渲染的屏幕分辨率，这个并不会影响渲染区域的大小，只是字体的相对大小，可以不用改动
width = 1280
height = 720

# 使用字体名称
font = "微软雅黑"

# 弹幕字体大小
font_size = 25

# 下面两个数值用来计算弹幕的【水平距离】
# width_ratio 是一个比例数，用来计算平衡不同字体的宽度。
#   有的字体比较粗、比较宽，可以适当调大 width_ratio（如 1.4、1.6），
#   有的字体比较细、比较窄，可以适当调小 width_ratio（如 1.0、1.2）。
# horizontal_gap 用来调整弹幕时间的水平距离，单位是像素，如果想拉开弹幕之间的距离，可以调大 horizontal_gap
width_ratio = 1.2
horizontal_gap = 5.0

# 计算弹幕高度的数值，即【行高度/行间距】。数值越大，弹幕的垂直距离越大
lane_size = 25

# 弹幕在屏幕上的【持续时间】，单位为秒，可以有小数
duration = 12

# 【正常弹幕的屏幕填充占比】，默认为 50%，即“半屏填充”。
float_percentage = 1

# 弹幕的不透明度，越小越透明，越大越不透明
alpha = 1

# 默认会跳过 ass 比 xml 修改时间更晚的文件，修改此参数为 true 会强制转换
force = true

# 黑名单，需要过滤的关键词列表文件，每行一个关键词
# denylist = "黑名单.txt"

# 在处理完后暂停等待输入
pause = false

# 弹幕的描边宽度，单位为像素
outline = 1

# 弹幕的阴影宽度，单位为像素
shadow = 1

# 字体是否加粗
bold = false
```

## 命令行
如果你熟悉命令行，可以直接使用：

```plaintext
danmu2ass 0.2.2
gwy15
将 XML 弹幕转换为 ASS 文件

USAGE:
    danmu2ass.exe [OPTIONS] [INPUT]

ARGS:
    <INPUT>    需要转换的输入，可以是 xml 文件、文件夹或是哔哩哔哩链接、BV
               号。如果是文件夹会递归将其下所有 XML 都进行转换 [default: .]

OPTIONS:
    -a, --alpha <ALPHA>
            弹幕不透明度 [default: 1]

        --bold
            加粗

    -d, --duration <DURATION>
            弹幕在屏幕上的持续时间，单位为秒，可以有小数 [default: 12]

        --denylist <DENYLIST>
            黑名单，需要过滤的关键词列表文件，每行一个关键词

    -f, --font <FONT>
            弹幕使用字体 [default: 微软雅黑]

        --font-size <FONT_SIZE>
            弹幕字体大小 [default: 25]

        --force <force>
            默认会跳过 ass 比 xml 修改时间更晚的文件，此参数会强制转换 [default: true]

    -h, --height <HEIGHT>
            屏幕高度 [default: 720]

        --help
            Print help information

        --horizontal-gap <HORIZONTAL_GAP>
            每条弹幕之间的最小水平间距，为避免重叠可以调大这个数值。单位：像素 [default: 5.0]

    -l, --lane-size <LANE_SIZE>
            弹幕所占据的高度，即“行高度/行间距” [default: 25]

    -o, --output <ASS_FILE>
            输出的 ASS 文件，默认为输入文件名将 .xml 替换为 .ass，如果输入是文件夹则忽略

        --outline <OUTLINE>
            描边宽度 [default: 1]

    -p, --float-percentage <FLOAT_PERCENTAGE>
            屏幕上滚动弹幕最多高度百分比 [default: 1]

        --pause
            在处理完后暂停等待输入

        --shadow <SHADOW>
            阴影 [default: 1]

        --time-offset <TIME_OFFSET>
            时间轴偏移，>0 会让弹幕延后，<0 会让弹幕提前，单位为秒 [default: 0.0]

    -V, --version
            Print version information

    -w, --width <WIDTH>
            屏幕宽度 [default: 1280]

        --width-ratio <WIDTH_RATIO>
            计算弹幕宽度的比例，为避免重叠可以调大这个数值 [default: 1.2]
```
