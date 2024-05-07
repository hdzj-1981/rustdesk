lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "状态"),
        ("Your Desktop", "你的桌面"),
        ("desk_tip", "你的桌面可以通过下面的 ID 和密码访问。"),
        ("Password", "密码"),
        ("Ready", "就绪"),
        ("Established", "已建立"),
        ("connecting_status", "正在接入 RustDesk 网络..."),
        ("Enable service", "允许服务"),
        ("Start service", "启动服务"),
        ("Service is running", "服务正在运行"),
        ("Service is not running", "服务未运行"),
        ("not_ready_status", "未就绪，请检查网络连接"),
        ("Control Remote Desktop", "控制远程桌面"),
        ("Transfer file", "传输文件"),
        ("Connect", "连接"),
        ("Recent sessions", "最近访问过"),
        ("Address book", "地址簿"),
        ("Confirmation", "确认"),
        ("TCP tunneling", "TCP 隧道"),
        ("Remove", "删除"),
        ("Refresh random password", "刷新随机密码"),
        ("Set your own password", "设置密码"),
        ("Enable keyboard/mouse", "允许控制键盘/鼠标"),
        ("Enable clipboard", "允许同步剪贴板"),
        ("Enable file transfer", "允许传输文件"),
        ("Enable TCP tunneling", "允许建立 TCP 隧道"),
        ("IP Whitelisting", "IP 白名单"),
        ("ID/Relay Server", "ID/中继服务器"),
        ("Import server config", "导入服务器配置"),
        ("Export Server Config", "导出服务器配置"),
        ("Import server configuration successfully", "导入服务器配置信息成功"),
        ("Export server configuration successfully", "导出服务器配置信息成功"),
        ("Invalid server configuration", "服务器配置无效，请修改后重新复制配置信息到剪贴板，然后点击此按钮"),
        ("Clipboard is empty", "复制配置信息到剪贴板后点击此按钮，可以自动导入配置"),
        ("Stop service", "停止服务"),
        ("Change ID", "更改 ID"),
        ("Your new ID", "你的新 ID"),
        ("length %min% to %max%", "长度在 %min% 与 %max% 之间"),
        ("starts with a letter", "以字母开头"),
        ("allowed characters", "使用允许的字符"),
        ("id_change_tip", "只可以使用字母 a-z, A-Z, 0-9, _ (下划线)。首字母必须是 a-z, A-Z。长度在 6 与 16 之间。"),
        ("Website", "网站"),
        ("About", "关于"),
        ("Slogan_tip", ""),
        ("Privacy Statement", "隐私声明"),
        ("Mute", "静音"),
        ("Build Date", "构建日期"),
        ("Version", "版本"),
        ("Home", "主页"),
        ("Audio Input", "音频输入"),
        ("Enhancements", "增强功能"),
        ("Hardware Codec", "硬件编解码"),
        ("Adaptive bitrate", "自适应码率"),
        ("ID Server", "ID 服务器"),
        ("Relay Server", "中继服务器"),
        ("API Server", "API 服务器"),
        ("invalid_http", "必须以 http:// 或者 https:// 开头"),
        ("Invalid IP", "无效 IP"),
        ("Invalid format", "无效格式"),
        ("server_not_support", "服务器暂不支持"),
        ("Not available", "不可用"),
        ("Too frequent", "修改太频繁，请稍后再试"),
        ("Cancel", "取消"),
        ("Skip", "跳过"),
        ("Close", "关闭"),
        ("Retry", "再试"),
        ("OK", "确认"),
        ("Password Required", "需要密码"),
        ("Please enter your password", "请输入密码"),
        ("Remember password", "记住密码"),
        ("Wrong Password", "密码错误"),
        ("Do you want to enter again?", "是否要再次输入？"),
        ("Connection Error", "连接错误"),
        ("Error", "错误"),
        ("Reset by the peer", "连接被对方关闭"),
        ("Connecting...", "正在连接..."),
        ("Connection in progress. Please wait.", "正在进行连接，请稍候。"),
        ("Please try 1 minute later", "一分钟后再试"),
        ("Login Error", "登录错误"),
        ("Successful", "成功"),
        ("Connected, waiting for image...", "已连接，等待画面传输..."),
        ("Name", "名称"),
        ("Type", "类型"),
        ("Modified", "修改时间"),
        ("Size", "大小"),
        ("Show Hidden Files", "显示隐藏文件"),
        ("Receive", "接受"),
        ("Send", "发送"),
        ("Refresh File", "刷新文件"),
        ("Local", "本地"),
        ("Remote", "远程"),
        ("Remote Computer", "远程电脑"),
        ("Local Computer", "本地电脑"),
        ("Confirm Delete", "确认删除"),
        ("Delete", "删除"),
        ("Properties", "属性"),
        ("Multi Select", "多选"),
        ("Select All", "全选"),
        ("Unselect All", "取消全选"),
        ("Empty Directory", "空文件夹"),
        ("Not an empty directory", "这不是一个空文件夹"),
        ("Are you sure you want to delete this file?", "是否删除此文件？"),
        ("Are you sure you want to delete this empty directory?", "是否删除此空文件夹？"),
        ("Are you sure you want to delete the file of this directory?", "是否删除此文件夹下的文件？"),
        ("Do this for all conflicts", "应用于其它冲突"),
        ("This is irreversible!", "此操作不可逆！"),
        ("Deleting", "正在删除"),
        ("files", "文件"),
        ("Waiting", "正在等待..."),
        ("Finished", "完成"),
        ("Speed", "速度"),
        ("Custom Image Quality", "设置画面质量"),
        ("Privacy mode", "隐私模式"),
        ("Block user input", "阻止用户输入"),
        ("Unblock user input", "取消阻止用户输入"),
        ("Adjust Window", "调节窗口"),
        ("Original", "原始比例"),
        ("Shrink", "收缩"),
        ("Stretch", "伸展"),
        ("Scrollbar", "滚动条"),
        ("ScrollAuto", "自动滚动"),
        ("Good image quality", "画质最优化"),
        ("Balanced", "平衡"),
        ("Optimize reaction time", "速度最优化"),
        ("Custom", "自定义"),
        ("Show remote cursor", "显示远程光标"),
        ("Show quality monitor", "显示质量监测"),
        ("Disable clipboard", "禁用剪贴板"),
        ("Lock after session end", "会话结束后锁定远程电脑"),
        ("Insert", "插入"),
        ("Insert Lock", "锁定远程电脑"),
        ("Refresh", "刷新画面"),
        ("ID does not exist", "ID 不存在"),
        ("Failed to connect to rendezvous server", "连接注册服务器失败"),
        ("Please try later", "请稍后再试"),
        ("Remote desktop is offline", "远程电脑处于离线状态"),
        ("Key mismatch", "Key 不匹配"),
        ("Timeout", "连接超时"),
        ("Failed to connect to relay server", "无法连接到中继服务器"),
        ("Failed to connect via rendezvous server", "无法通过注册服务器建立连接"),
        ("Failed to connect via relay server", "无法通过中继服务器建立连接"),
        ("Failed to make direct connection to remote desktop", "无法直接连接到远程桌面"),
        ("Set Password", "设置密码"),
        ("OS Password", "操作系统密码"),
        ("install_tip", "你正在运行未安装版本，由于 UAC 限制，作为被控端，会在某些情况下无法控制鼠标键盘，或者录制屏幕，请点击下面的按钮将 RustDesk 安装到系统，从而规避上述问题。"),
        ("Click to upgrade", "点击这里升级"),
        ("Click to download", "点击这里下载"),
        ("Click to update", "点击这里更新"),
        ("Configure", "配置"),
        ("config_acc", "为了能够远程控制你的桌面, 请给予 RustDesk \"辅助功能\" 权限。"),
        ("config_screen", "为了能够远程访问你的桌面, 请给予 RustDesk \"屏幕录制\" 权限。"),
        ("Installing ...", "安装中..."),
        ("Install", "安装"),
        ("Installation", "安装"),
        ("Installation Path", "安装路径"),
        ("Create start menu shortcuts", "创建启动菜单快捷方式"),
        ("Create desktop icon", "创建桌面图标"),
        ("agreement_tip", "开始安装即表示接受许可协议。"),
        ("Accept and Install", "同意并安装"),
        ("End-user license agreement", "用户协议"),
        ("Generating ...", "正在生成..."),
        ("Your installation is lower version.", "你安装的版本比当前运行的低。"),
        ("not_close_tcp_tip", "请在使用隧道的时候，不要关闭本窗口"),
        ("Listening ...", "正在等待隧道连接..."),
        ("Remote Host", "远程主机"),
        ("Remote Port", "远程端口"),
        ("Action", "动作"),
        ("Add", "添加"),
        ("Local Port", "本地端口"),
        ("Local Address", "当前地址"),
        ("Change Local Port", "修改本地端口"),
        ("setup_server_tip", "如果需要更快连接速度，你可以选择自建服务器"),
        ("Too short, at least 6 characters.", "太短了，至少 6 个字符"),
        ("The confirmation is not identical.", "两次输入不匹配"),
        ("Permissions", "权限"),
        ("Accept", "接受"),
        ("Dismiss", "拒绝"),
        ("Disconnect", "断开连接"),
        ("Enable file copy and paste", "允许复制粘贴文件"),
        ("Connected", "已连接"),
        ("Direct and encrypted connection", "加密直连"),
        ("Relayed and encrypted connection", "加密中继连接"),
        ("Direct and unencrypted connection", "非加密直连"),
        ("Relayed and unencrypted connection", "非加密中继连接"),
        ("Enter Remote ID", "输入对方 ID"),
        ("Enter your password", "输入密码"),
        ("Logging in...", "正在登录..."),
        ("Enable RDP session sharing", "允许 RDP 会话共享"),
        ("Auto Login", "自动登录（设置断开后锁定才有效）"),
        ("Enable direct IP access", "允许 IP 直接访问"),
        ("Rename", "重命名"),
        ("Space", "空格"),
        ("Create desktop shortcut", "创建桌面快捷方式"),
        ("Change Path", "更改路径"),
        ("Create Folder", "创建文件夹"),
        ("Please enter the folder name", "请输入文件夹名称"),
        ("Fix it", "修复"),
        ("Warning", "警告"),
        ("Login screen using Wayland is not supported", "不支持使用 Wayland 登录界面"),
        ("Reboot required", "重启后才能生效"),
        ("Unsupported display server", "不支持当前显示服务器"),
        ("x11 expected", "请切换到 x11"),
        ("Port", "端口"),
        ("Settings", "设置"),
        ("Username", "用户名"),
        ("Invalid port", "无效端口"),
        ("Closed manually by the peer", "被对方手动关闭"),
        ("Enable remote configuration modification", "允许远程修改配置"),
        ("Run without install", "不安装直接运行"),
        ("Connect via relay", "中继连接"),
        ("Always connect via relay", "强制走中继连接"),
        ("whitelist_tip", "只有白名单里的 IP 才能访问本机"),
        ("Login", "登录"),
        ("Verify", "验证"),
        ("Remember me", "记住我"),
        ("Trust this device", "信任此设备"),
        ("Verification code", "验证码"),
        ("verification_tip", "已向注册邮箱发送了登录验证码，请输入验证码继续登录"),
        ("Logout", "登出"),
        ("Tags", "标签"),
        ("Search ID", "查找 ID"),
        ("whitelist_sep", "可以使用逗号，分号，空格或者换行符作为分隔符"),
        ("Add ID", "增加 ID"),
        ("Add Tag", "增加标签"),
        ("Unselect all tags", "取消选择所有标签"),
        ("Network error", "网络错误"),
        ("Username missed", "用户名没有填写"),
        ("Password missed", "密码没有填写"),
        ("Wrong credentials", "提供的登录信息错误"),
        ("The verification code is incorrect or has expired", "验证码错误或已超时"),
        ("Edit Tag", "修改标签"),
        ("Forget Password", "忘记密码"),
        ("Favorites", "收藏"),
        ("Add to Favorites", "加入到收藏"),
        ("Remove from Favorites", "从收藏中删除"),
        ("Empty", "空空如也"),
        ("Invalid folder name", "无效文件夹名称"),
        ("Socks5 Proxy", "Socks5 代理"),
        ("Socks5/Http(s) Proxy", "Socks5/Http(s) 代理"),
        ("Discovered", "已发现"),
        ("install_daemon_tip", "为了开机启动，请安装系统服务。"),
        ("Remote ID", "远程 ID"),
        ("Paste", "粘贴"),
        ("Paste here?", "粘贴到这里？"),
        ("Are you sure to close the connection?", "是否确认关闭连接？"),
        ("Download new version", "下载新版本"),
        ("Touch mode", "触屏模式"),
        ("Mouse mode", "鼠标模式"),
        ("One-Finger Tap", "单指轻触"),
        ("Left Mouse", "鼠标左键"),
        ("One-Long Tap", "单指长按"),
        ("Two-Finger Tap", "双指轻触"),
        ("Right Mouse", "鼠标右键"),
        ("One-Finger Move", "单指移动"),
        ("Double Tap & Move", "双击并移动"),
        ("Mouse Drag", "鼠标选中拖动"),
        ("Three-Finger vertically", "三指垂直滑动"),
        ("Mouse Wheel", "鼠标滚轮"),
        ("Two-Finger Move", "双指移动"),
        ("Canvas Move", "移动画布"),
        ("Pinch to Zoom", "双指缩放"),
        ("Canvas Zoom", "缩放画布"),
        ("Reset canvas", "重置画布"),
        ("No permission of file transfer", "没有文件传输权限"),
        ("Note", "备注"),
        ("Connection", "连接"),
        ("Share Screen", "共享屏幕"),
        ("Chat", "聊天消息"),
        ("Total", "总计"),
        ("items", "个项目"),
        ("Selected", "已选择"),
        ("Screen Capture", "屏幕录制"),
        ("Input Control", "输入控制"),
        ("Audio Capture", "音频录制"),
        ("File Connection", "文件连接"),
        ("Screen Connection", "屏幕连接"),
        ("Do you accept?", "是否接受？"),
        ("Open System Setting", "打开系统设置"),
        ("How to get Android input permission?", "如何获取安卓的输入权限？"),
        ("android_input_permission_tip1", "为了让远程设备通过鼠标或触屏控制您的安卓设备，你需要允许 RustDesk 使用\"无障碍\"服务。"),
        ("android_input_permission_tip2", "请在接下来的系统设置页面里，找到并进入 [已安装的服务] 页面，将 [RustDesk Input] 服务开启。"),
        ("android_new_connection_tip", "收到新的连接控制请求，对方想要控制你当前的设备。"),
        ("android_service_will_start_tip", "开启录屏权限将自动开启服务，允许其他设备向此设备请求建立连接。"),
        ("android_stop_service_tip", "关闭服务将自动关闭所有已建立的连接。"),
        ("android_version_audio_tip", "当前安卓版本不支持音频录制，请升级至安卓 10 或更高。"),
        ("android_start_service_tip", "点击开始服务或启用屏幕捕获权限，即可启动屏幕共享服务"),
        ("android_permission_may_not_change_tip", "对于已建立的连接，权限可能不会立即发生改变，除非重新建立连接。"),
        ("Account", "账户"),
        ("Overwrite", "覆盖"),
        ("This file exists, skip or overwrite this file?", "这个文件/文件夹已存在，跳过/覆盖？"),
        ("Quit", "退出"),
        ("Help", "帮助"),
        ("Failed", "失败"),
        ("Succeeded", "成功"),
        ("Someone turns on privacy mode, exit", "其他用户使用隐私模式，退出"),
        ("Unsupported", "不支持"),
        ("Peer denied", "被控端拒绝"),
        ("Please install plugins", "请安装插件"),
        ("Peer exit", "被控端退出"),
        ("Failed to turn off", "退出失败"),
        ("Turned off", "退出"),
        ("Language", "语言"),
        ("Keep RustDesk background service", "保持 RustDesk 后台服务"),
        ("Ignore Battery Optimizations", "忽略电池优化"),
        ("android_open_battery_optimizations_tip", "如需关闭此功能，请在接下来的 RustDesk 应用设置页面中，找到并进入 [电源] 页面，取消勾选 [不受限制]"),
        ("Start on boot", "开机自启动"),
        ("Start the screen sharing service on boot, requires special permissions", "开机自动启动屏幕共享服务，此功能需要一些特殊权限。"),
        ("Connection not allowed", "对方不允许连接"),
        ("Legacy mode", "传统模式"),
        ("Map mode", "1：1 传输"),
        ("Translate mode", "翻译模式"),
        ("Use permanent password", "使用固定密码"),
        ("Use both passwords", "同时使用两种密码"),
        ("Set permanent password", "设置固定密码"),
        ("Enable remote restart", "允许远程重启"),
        ("Restart remote device", "重启远程电脑"),
        ("Are you sure you want to restart", "确定要重启"),
        ("Restarting remote device", "正在重启远程设备"),
        ("remote_restarting_tip", "远程设备正在重启, 请关闭当前提示框, 并在一段时间后使用永久密码重新连接"),
        ("Copied", "已复制"),
        ("Exit Fullscreen", "退出全屏"),
        ("Fullscreen", "全屏"),
        ("Mobile Actions", "移动端操作"),
        ("Select Monitor", "选择监视器"),
        ("Control Actions", "控制操作"),
        ("Display Settings", "显示设置"),
        ("Ratio", "比例"),
        ("Image Quality", "画质"),
        ("Scroll Style", "滚屏方式"),
        ("Show Toolbar", "显示工具栏"),
        ("Hide Toolbar", "隐藏工具栏"),
        ("Direct Connection", "直接连接"),
        ("Relay Connection", "中继连接"),
        ("Secure Connection", "安全连接"),
        ("Insecure Connection", "非安全连接"),
        ("Scale original", "原始尺寸"),
        ("Scale adaptive", "适应窗口"),
        ("General", "常规"),
        ("Security", "安全"),
        ("Theme", "主题"),
        ("Dark Theme", "暗黑主题"),
        ("Light Theme", "明亮主题"),
        ("Dark", "黑暗"),
        ("Light", "明亮"),
        ("Follow System", "跟随系统"),
        ("Enable hardware codec", "使能硬件编解码"),
        ("Unlock Security Settings", "解锁安全设置"),
        ("Enable audio", "允许传输音频"),
        ("Unlock Network Settings", "解锁网络设置"),
        ("Server", "服务器"),
        ("Direct IP Access", "IP 直接访问"),
        ("Proxy", "代理"),
        ("Apply", "应用"),
        ("Disconnect all devices?", "断开所有远程连接？"),
        ("Clear", "清空"),
        ("Audio Input Device", "音频输入设备"),
        ("Use IP Whitelisting", "只允许白名单上的 IP 访问"),
        ("Network", "网络"),
        ("Pin Toolbar", "固定工具栏"),
        ("Unpin Toolbar", "取消固定工具栏"),
        ("Recording", "录屏"),
        ("Directory", "目录"),
        ("Automatically record incoming sessions", "自动录制来访会话"),
        ("Change", "更改"),
        ("Start session recording", "开始录屏"),
        ("Stop session recording", "结束录屏"),
        ("Enable recording session", "允许录制会话"),
        ("Enable LAN discovery", "允许局域网发现"),
        ("Deny LAN discovery", "拒绝局域网发现"),
        ("Write a message", "输入聊天消息"),
        ("Prompt", "提示"),
        ("Please wait for confirmation of UAC...", "请等待对方确认 UAC..."),
        ("elevated_foreground_window_tip", "远端桌面的当前窗口需要更高的权限才能操作, 暂时无法使用鼠标键盘, 可以请求对方最小化当前窗口, 或者在连接管理窗口点击提升。为避免这个问题，建议在远端设备上安装本软件。"),
        ("Disconnected", "会话已结束"),
        ("Other", "其他"),
        ("Confirm before closing multiple tabs", "关闭多个标签页时向您确认"),
        ("Keyboard Settings", "键盘设置"),
        ("Full Access", "完全访问"),
        ("Screen Share", "仅共享屏幕"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland 需要 Ubuntu 21.04 或更高版本。"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland 需要更高版本的 linux 发行版。 请尝试 X11 桌面或更改您的操作系统。"),
        ("JumpLink", "查看"),
        ("Please Select the screen to be shared(Operate on the peer side).", "请选择要分享的画面（对端操作）。"),
        ("Show RustDesk", "显示 RustDesk"),
        ("This PC", "此电脑"),
        ("or", "或"),
        ("Continue with", "使用"),
        ("Elevate", "提权"),
        ("Zoom cursor", "缩放光标"),
        ("Accept sessions via password", "只允许密码访问"),
        ("Accept sessions via click", "只允许点击访问"),
        ("Accept sessions via both", "允许密码或点击访问"),
        ("Please wait for the remote side to accept your session request...", "请等待对方接受你的连接..."),
        ("One-time Password", "一次性密码"),
        ("Use one-time password", "使用一次性密码"),
        ("One-time password length", "一次性密码长度"),
        ("Request access to your device", "请求访问你的设备"),
        ("Hide connection management window", "隐藏连接管理窗口"),
        ("hide_cm_tip", "在只允许密码连接并且只用固定密码的情况下才允许隐藏"),
        ("wayland_experiment_tip", "Wayland 支持处于实验阶段，如果你需要使用无人值守访问，请使用 X11。"),
        ("Right click to select tabs", "右键选择选项卡"),
        ("Skipped", "已跳过"),
        ("Add to address book", "添加到地址簿"),
        ("Group", "小组"),
        ("Search", "搜索"),
        ("Closed manually by web console", "被 web 控制台手动关闭"),
        ("Local keyboard type", "本地键盘类型"),
        ("Select local keyboard type", "请选择本地键盘类型"),
        ("software_render_tip", "如果你使用英伟达显卡, 并且远程窗口在会话建立后会立刻关闭, 那么安装 nouveau 驱动并且选择使用软件渲染可能会有帮助。重启软件后生效。"),
        ("Always use software rendering", "始终使用软件渲染"),
        ("config_input", "为了能够通过键盘控制远程桌面, 请给予 RustDesk \"输入监控\" 权限。"),
        ("config_microphone", "为了支持通过麦克风进行音频传输，请给予 RustDesk \"录音\"权限。"),
        ("request_elevation_tip", "如果对面有人, 也可以请求提升权限。"),
        ("Wait", "等待"),
        ("Elevation Error", "提权失败"),
        ("Ask the remote user for authentication", "请求远端用户授权"),
        ("Choose this if the remote account is administrator", "当对面电脑是管理员账号时选择该选项"),
        ("Transmit the username and password of administrator", "发送管理员账号的用户名密码"),
        ("still_click_uac_tip", "依然需要被控端用户在运行 RustDesk 的 UAC 窗口点击确认。"),
        ("Request Elevation", "请求提权"),
        ("wait_accept_uac_tip", "请等待远端用户确认 UAC 对话框。"),
        ("Elevate successfully", "提权成功"),
        ("uppercase", "大写字母"),
        ("lowercase", "小写字母"),
        ("digit", "数字"),
        ("special character", "特殊字符"),
        ("length>=8", "长度不小于 8"),
        ("Weak", "弱"),
        ("Medium", "中"),
        ("Strong", "强"),
        ("Switch Sides", "反转访问方向"),
        ("Please confirm if you want to share your desktop?", "请确认是否要让对方访问你的桌面？"),
        ("Display", "显示"),
        ("Default View Style", "默认显示方式"),
        ("Default Scroll Style", "默认滚动方式"),
        ("Default Image Quality", "默认图像质量"),
        ("Default Codec", "默认编解码"),
        ("Bitrate", "码率"),
        ("FPS", "帧率"),
        ("Auto", "自动"),
        ("Other Default Options", "其它默认选项"),
        ("Voice call", "语音通话"),
        ("Text chat", "文字聊天"),
        ("Stop voice call", "停止语音通话"),
        ("relay_hint_tip", "可能无法直连，可以尝试中继连接。\n另外，如果想直接使用中继连接，可以在 ID 后面添加/r，如果最近访问里存在该卡片，也可以在卡片选项里选择强制走中继连接。"),
        ("Reconnect", "重连"),
        ("Codec", "编解码"),
        ("Resolution", "分辨率"),
        ("No transfers in progress", "无进行中的传输"),
        ("Set one-time password length", "设置一次性密码长度"),
        ("RDP Settings", "RDP 设置"),
        ("Sort by", "排序方式"),
        ("New Connection", "新连接"),
        ("Restore", "恢复"),
        ("Minimize", "最小化"),
        ("Maximize", "最大化"),
        ("Your Device", "你的设备"),
        ("empty_recent_tip", "无最近会话，是时候开始新会话了！"),
        ("empty_favorite_tip", "还没有收藏的被控端？找一个人连接并将其添加到收藏吧！"),
        ("empty_lan_tip", "情况不妙，似乎未发现任何被控端！"),
        ("empty_address_book_tip", "似乎目前地址簿内无被控端"),
        ("eg: admin", "例如：admin"),
        ("Empty Username", "空用户名"),
        ("Empty Password", "空密码"),
        ("Me", "我"),
        ("identical_file_tip", "此文件与对方的一致"),
        ("show_monitors_tip", "在工具栏上显示监视器"),
        ("View Mode", "浏览模式"),
        ("login_linux_tip", "登录被控端的 Linux 账户，才能启用 X 桌面"),
        ("verify_rustdesk_password_tip", "验证 RustDesk 密码"),
        ("remember_account_tip", "记住此账户"),
        ("os_account_desk_tip", "在无显示器的环境下，此账户用于登录被控系统，并启用桌面"),
        ("OS Account", "系统账户"),
        ("another_user_login_title_tip", "其他用户已登录"),
        ("another_user_login_text_tip", "断开"),
        ("xorg_not_found_title_tip", "Xorg 未安装"),
        ("xorg_not_found_text_tip", "请安装 Xorg"),
        ("no_desktop_title_tip", "desktop 未安装"),
        ("no_desktop_text_tip", "请安装 desktop"),
        ("No need to elevate", "无需提升权限"),
        ("System Sound", "系统音频"),
        ("Default", "默认"),
        ("New RDP", "新 RDP 连接"),
        ("Fingerprint", "指纹"),
        ("Copy Fingerprint", "复制指纹"),
        ("no fingerprints", "没有指纹"),
        ("Select a peer", "选择一个被控端"),
        ("Select peers", "选择被控"),
        ("Plugins", "插件"),
        ("Uninstall", "卸载"),
        ("Update", "更新"),
        ("Enable", "启用"),
        ("Disable", "禁用"),
        ("Options", "选项"),
        ("resolution_original_tip", "原始分辨率"),
        ("resolution_fit_local_tip", "适应本地分辨率"),
        ("resolution_custom_tip", "自定义分辨率"),
        ("Collapse toolbar", "折叠工具栏"),
        ("Accept and Elevate", "接受并提权"),
        ("accept_and_elevate_btn_tooltip", "接受连接并提升 UAC 权限"),
        ("clipboard_wait_response_timeout_tip", "等待拷贝响应超时"),
        ("Incoming connection", "收到的连接"),
        ("Outgoing connection", "发起的连接"),
        ("Exit", "退出"),
        ("Open", "打开"),
        ("logout_tip", "确定要退出登录吗？"),
        ("Service", "服务"),
        ("Start", "启动"),
        ("Stop", "停止"),
        ("exceed_max_devices", "管理的设备数已达到最大值"),
        ("Sync with recent sessions", "同步最近会话"),
        ("Sort tags", "对标签进行排序"),
        ("Open connection in new tab", "在选项卡中打开新连接"),
        ("Move tab to new window", "将标签页移至新窗口"),
        ("Can not be empty", "不能为空"),
        ("Already exists", "已经存在"),
        ("Change Password", "更改密码"),
        ("Refresh Password", "刷新密码"),
        ("ID", "ID"),
        ("Grid View", "网格视图"),
        ("List View", "列表视图"),
        ("Select", "选择"),
        ("Toggle Tags", "切换标签"),
        ("pull_ab_failed_tip", "获取地址簿失败"),
        ("push_ab_failed_tip", "上传地址簿失败"),
        ("synced_peer_readded_tip", "最近会话中存在的设备将会被重新同步到地址簿。"),
        ("Change Color", "更改颜色"),
        ("Primary Color", "基本色"),
        ("HSV Color", "HSV 色"),
        ("Installation Successful!", "安装成功!"),
        ("Installation failed!", "安装失败!"),
        ("Reverse mouse wheel", "鼠标滚轮反向"),
        ("{} sessions", "{} 个会话"),
        ("scam_title", "你可能被骗了！"),
        ("scam_text1", "如果你正在和素不相识的人通话，而对方要求你使用 RustDesk 启动服务，请勿继续操作并立刻挂断电话。"),
        ("scam_text2", "他们很可能是骗子，试图窃取您的钱财或其他个人信息。"),
        ("Don't show again", "下次不再显示"),
        ("I Agree", "同意"),
        ("Decline", "拒绝"),
        ("Timeout in minutes", "超时（分钟）"),
        ("auto_disconnect_option_tip", "自动关闭不活跃的会话"),
        ("Connection failed due to inactivity", "由于长时间无操作, 连接被自动断开"),
        ("Check for software update on startup", "启动时检查软件更新"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "请升级专业版服务器到{}或更高版本！"),
        ("pull_group_failed_tip", "获取组信息失败"),
        ("Filter by intersection", "按交集过滤"),
        ("Remove wallpaper during incoming sessions", "接受会话时移除桌面壁纸"),
        ("Test", "测试"),
        ("display_is_plugged_out_msg", "显示器被拔出，切换到第一个显示器。"),
        ("No displays", "没有显示器。"),
        ("elevated_switch_display_msg", "切换到主显示器，因为用户提权后，不支持多显示器画面。"),
        ("Open in new window", "在新的窗口中打开"),
        ("Show displays as individual windows", "在单个窗口中打开显示器"),
        ("Use all my displays for the remote session", "将我的所有显示器用于远程会话"),
        ("selinux_tip", "SELinux 处于启用状态，RustDesk 可能无法作为被控正常运行。"),
        ("Change view", "更改视图"),
        ("Big tiles", "大磁贴"),
        ("Small tiles", "小磁贴"),
        ("List", "列表"),
        ("Virtual display", "虚拟显示器"),
        ("Plug out all", "拔出所有"),
        ("True color (4:4:4)", "真彩模式（4:4:4）"),
        ("Enable blocking user input", "允许阻止用户输入"),
        ("id_input_tip", "可以输入 ID、直连 IP，或域名和端口号（<域名>:<端口号>）。\n要访问另一台服务器上的设备，请附加服务器地址（<ID>@<服务器地址>?key=<密钥>）。比如，\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=。\n要访问公共服务器上的设备，请输入 \"<ID>@public\", 无需密钥。\n\n如果您想要在首次连接时，强制走中继连接，请在 ID 的后面添加 \"/r\"，例如，\"9123456234/r\"。"),
        ("privacy_mode_impl_mag_tip", "模式 1"),
        ("privacy_mode_impl_virtual_display_tip", "模式 2"),
        ("Enter privacy mode", "进入隐私模式"),
        ("Exit privacy mode", "退出隐私模式"),
        ("idd_not_support_under_win10_2004_tip", "不支持 Indirect display driver 。需要 Windows 10 版本 2004 及更高的版本。"),
        ("switch_display_elevated_connections_tip", "用户提权后，被控有多个连接，不能切换到非主显示器。若要控制多显示器，请安装后再试。"),
        ("input_source_1_tip", "输入源 1"),
        ("input_source_2_tip", "输入源 2"),
        ("capture_display_elevated_connections_tip", "用户提权后，不能显示多个显示器。若要控制多显示器，请安装后再试。"),
        ("Swap control-command key", "交换 Control 键和 Command 键"),
        ("swap-left-right-mouse", "交换鼠标左右键"),
        ("2FA code", "双重认证代码"),
        ("More", "更多"),
        ("enable-2fa-title", "启用双重认证"),
        ("enable-2fa-desc", "现在请设置身份验证器。您可以在手机或台式电脑上使用 Authy、Microsoft 或 Google Authenticator 等验证器。用验证器扫描二维码，然后输入显示的验证码以启用双重认证。"),
        ("wrong-2fa-code", "无法验证此验证码。请检查验证码和本地时间设置是否正确。"),
        ("enter-2fa-title", "双重认证"),
        ("Email verification code must be 6 characters.", "Email 验证码必须是 6 个字符。"),
        ("2FA code must be 6 digits.", "双重认证代码必须是 6 位数字。"),
        ("Multiple Windows sessions found", "发现多个 Windows 会话"),
        ("Please select the session you want to connect to", "请选择您要连接的会话"),
        ("powered_by_me", "由 RustDesk 提供支持"),
        ("outgoing_only_desk_tip", "当前版本的软件是定制版本。\n您可以连接至其他设备，但是其他设备无法连接至您的设备。"),
        ("preset_password_warning", "此定制版本附有预设密码。 任何知晓此密码的人都能完全控制您的设备。如果这不是您所预期的，请立即卸载此软件。"),
        ("Security Alert", "安全警告"),
        ("My address book", "我的地址簿"),
        ("Personal", "个人的"),
        ("Owner", "所有者"),
        ("Set shared password", "设置共享密码"),
        ("Exist in", "存在于"),
        ("Read-only", "只读"),
        ("Read/Write", "读写"),
        ("Full Control", "完全控制"),
        ("share_warning_tip", "上述的字段為共享且对其他人可见。"),
        ("Everyone", "所有人"),
        ("ab_web_console_tip", "打开 Web 控制台以执行更多操作"),
        ("allow-only-conn-window-open-tip", "仅当 RustDesk 窗口打开时允许连接"),
        ("no_need_privacy_mode_no_physical_displays_tip", "没有物理显示器，没必要使用隐私模式。"),
        ("Follow remote cursor", "跟随远程光标"),
        ("Follow remote window focus", "跟随远程窗口焦点"),
        ("default_proxy_tip", "默认代理协议及端口为 Socks5 和 1080"),
    ].iter().cloned().collect();
}
