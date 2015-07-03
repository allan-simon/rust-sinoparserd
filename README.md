# sinoparserd (in Rust)

## Run it

simply launch the binary `./sinoparserd`

## Use it

By default it opens an http connection on port 4000

Right now there's one api on

`POST /keywords` which expect this payload

```json
{

    'text' : 'some chinese text'
}

```

and it will return you the "keywords" of your text


```
{
    "10": 7, 
    "20": 2, 
    "一位": 2, 
    "世纪": 2, 
    "人物": 4, 
    "任": 2, 
    "但": 2, 
    "使用": 2, 
    "元": 5, 
    "决定": 2, 
    "出现": 3, 
    "登上": 3, 
    "目前": 2, 
    "称": 2, 
    "纸币": 4, 
    "美元": 7, 
    "美国": 10, 
    "美国第": 2, 
    "美钞": 9, 
    "表示": 2, 
    "设计": 2, 
    "评选": 2, 
    "财政部": 3, 
    "财长": 4, 
    "货币": 3, 
    "过": 3, 
    "重新设计": 2, 
    "领袖": 2, 
    "首次": 3
}
```

The value is the relevance score of the given Chinese word.

## Build it

sinoparserd is built against rust 1.0.0

To compile:

```
cargo build
```

## Break it

If you got unexpected result, if it crash etc. feel free to fill a bug report

## Fix it

PR are more than welcome, be it for bug fixing or adding new feature, I can guide you if you're new in rust etc.

## License it

MIT

## Data set

The data in words.json come from the project CC-CEDICT , which is under CC-BY-SA license
