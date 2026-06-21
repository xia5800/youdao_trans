# 优道翻译 (Youdao Translate)

> 一款简洁、高效、高颜值的输入、截图、划词翻译软件

不止翻译，更是词典。

## 功能

- **文本翻译** — 输入文本，在 9 种翻译源之间切换翻译
- **截图翻译** — 框选屏幕区域，OCR 识别后自动翻译
- **划词翻译** — 选中任意文字，快捷键触发弹窗翻译
- **词典查询** — 离线英汉词典，支持自动补全、词形变化、考试标签
- **翻译记录** — 历史记录自动保存，支持收藏、搜索、筛选、批量管理
- **系统托盘** — 后台常驻，托盘菜单快速访问各项功能
- **全局快捷键** — 自定义快捷键，全局可用
- **TTS 朗读** — 浏览器原生语音合成，支持多语种
- **深色模式** — 跟随系统 / 浅色 / 深色

## 截图

<!-- TODO: 添加截图 -->

## 下载

从 [Releases](../../releases) 页面下载最新版本安装包。

## 使用入门

### 基本操作

| 操作         | 方式                                      |
| ------------ | ----------------------------------------- |
| 打开翻译窗口 | 双击托盘图标，或托盘菜单 → 输入翻译       |
| 截图翻译     | 默认快捷键 `Alt+W`，或托盘菜单 → 截图翻译 |
| 划词翻译     | 选中文字后按 `Alt+T`（默认）              |
| 查单词       | 托盘菜单 → 查单词                         |
| 全局快捷键   | 设置 → 全局快捷键设置                     |

### 设置默认翻译源

打开设置 → 翻译源设置，选择一个翻译源并填写对应密钥（部分源无需密钥）。

### 自定义快捷键

打开设置 → 全局快捷键设置，点击快捷键输入框，按下新的组合键即可录制。

## 翻译源配置

### 谷歌翻译（无需密钥）

开箱即用，无需任何配置。依赖 Google 翻译公共接口，国内网络可能需要代理。

### 微软翻译（免费）

选择"微软翻译（免费）"，开箱即用，无需任何配置。
通过 Edge 翻译接口自动获取 Token，国内网络可能需要代理。

### 微软翻译（Azure）

1. 登录 [Azure Portal](https://portal.azure.com)
2. 创建 Translator 资源
3. 获取密钥（Key）和区域（Region，如 `eastasia`）
4. 在设置中填入：
   - `microsoft` → 密钥
   - `microsoft_region` → 区域

### OpenAI

1. 登录 [OpenAI Platform](https://platform.openai.com/)
2. 创建 API Key
3. 在设置中填入：
   - `openai_key` → API Key
   - `openai_model` → 模型名（可选，默认 `gpt-4o-mini`）
   - `openai_endpoint` → 自定义接口地址（可选，可用于代理或兼容 OpenAI 的服务）

### DeepLX

需自建 [DeepLX](https://github.com/OwO-Network/DeepLX) 服务。

1. 部署 DeepLX 服务（默认端口 1188）
2. 在设置中填入：
   - `deeplx_endpoint` → 接口地址（可选，默认 `http://localhost:1188`）

### Ollama（翻译）

需部署 [Ollama](https://ollama.com/) 并拉取翻译模型，如：

```bash
ollama pull granite3-dense:8b
```

在设置中填入：
- `ollama_base_url` → Ollama 地址（如 `http://localhost:11434`）
- `ollama_model` → 模型名（如 `granite3-dense:8b`）

### 百度翻译

1. 登录 [百度翻译开放平台](https://fanyi-api.baidu.com/)
2. 注册开发者，创建通用翻译 API 应用
3. 获取 App ID 和密钥
4. 在设置中填入：
   - `baidu_appid` → App ID
   - `baidu_appkey` → 密钥

### 阿里翻译

1. 登录 [阿里云官网](https://www.aliyun.com/)
2. 开通机器翻译服务（产品 → 人工智能 → 机器翻译）
3. 获取 AccessKey ID 和 AccessKey Secret
4. 在设置中填入：
   - `ali-accesskey` → AccessKey ID
   - `ali-secretkey` → AccessKey Secret

### 有道翻译

1. 登录 [有道智云](https://ai.youdao.com/)
2. 创建翻译服务应用
3. 获取应用 ID 和应用密钥
4. 在设置中填入：
   - `youdao_appid` → 应用 ID
   - `youdao_appsecret` → 应用密钥

## OCR 源配置

### Ollama OCR

需部署 [Ollama](https://ollama.com/) 并拉取视觉模型，如：

```bash
ollama pull llama3.2-vision
```

在设置中填入：
- `ollama_ocr_base_url` → Ollama 地址（如 `http://localhost:11434`）
- `ollama_ocr_model` → 视觉模型名（如 `llama3.2-vision`）

### 百度云 OCR

1. 登录 [百度智能云](https://console.bce.baidu.com/)
2. 创建 OCR 应用（文字识别 → 通用文字识别）
3. 获取 API Key 和 Secret Key
4. 在设置中填入：
   - `baidu_ocr-apikey` → API Key
   - `baidu_ocr-apisecret` → Secret Key

### 腾讯云 OCR

1. 登录 [腾讯云控制台](https://console.cloud.tencent.com/)
2. 开通 OCR 服务（产品 → 人工智能 → 文字识别）
3. 获取 SecretId 和 SecretKey
4. 在设置中填入：
   - `tencent-secretid` → SecretId
   - `tencent-secretkey` → SecretKey

### 讯飞 OCR

1. 登录 [讯飞开放平台](https://www.xfyun.cn/)
2. 创建 OCR 应用（通用文档识别）
3. 获取 App ID、API Key 和 API Secret
4. 在设置中填入：
   - `xunfei-appid` → App ID
   - `xunfei-apikey` → API Key
   - `xunfei-apisecret` → API Secret

## 配置说明

配置文件路径：`C:\Users\<用户名>\AppData\Roaming\youdao-fanyi\config\`

- **调试模式**：明文 JSON (`config-dev.json`)
- **发布模式**：AES-256-GCM 加密 (`config.enc`)，密钥由机器 UID + 盐值派生

历史数据库路径：`C:\Users\<用户名>\AppData\Roaming\youdao-fanyi\db\history.db`（调试模式为 `history-dev.db`）

## 开发

### 环境要求

- [Rust](https://www.rust-lang.org/) 1.77+
- [Node.js](https://nodejs.org/) 18+
- [Tauri CLI](https://v2.tauri.app/start/cli/)

### 启动开发环境

```bash
npm install          # 安装前端依赖
npm run tauri:dev    # 启动热重载开发
```

### 构建

以下命令面向开发者，需要先安装 [Rust](https://www.rust-lang.org/)、[Node.js](https://nodejs.org/) 18+、[Tauri CLI](https://v2.tauri.app/start/cli/)。

```bash
npm install          # 安装前端依赖
npm run tauri build  # 使用 Tauri 内置 NSIS 构建安装包
```

### 自定义安装包构建（NSIS）

如需使用项目中的自定义 NSIS 安装脚本（含 WebView2 检测、自定义安装路径等），在上述环境基础上还需安装 NSIS：

1. 下载 [NSIS 3.x](https://nsis.sourceforge.io/Download)
2. 安装（默认路径为 `C:\Program Files (x86)\NSIS`）
3. 运行 `build-installer.ps1` 即可生成安装包

```bash
.\build-installer.ps1
```

## 技术栈

- **前端**：Vue 3 + Vite + Vue Router
- **后端**：Rust + Tauri 2.x
- **数据库**：SQLite (字典 + 历史记录)
- **OCR**：百度云 / 腾讯云 / 讯飞 API
- **翻译**：微软 / 百度 / 阿里 / 有道 / 谷歌 API

## 其它

字典数据库地址：
https://github.com/skywind3000/ECDICT