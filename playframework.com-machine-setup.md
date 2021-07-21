# Playframework.com machine setup

## install the app

```
git clone git@github.com:playframework/playframework.com.git
```

### Install the actual source code

```
cd ~
git clone https://github.com/playframework/playframework.git playframework
cd playframework.com/data/
ln -s ~/playframework main
```

### Install translation files and generated docs

```
cd data
git clone https://github.com/playframework-ja/translation-project.git ja
git clone https://github.com/PlayFrameworkTR/translation-project tr
git clone https://github.com/cheleb/playframework-fr fr
git clone https://github.com/antonsarov/translation-project bg
git clone https://github.com/playframework/play-generated-docs.git generated
```