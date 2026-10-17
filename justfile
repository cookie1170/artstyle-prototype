web-target := 'web'
win-target := 'x86_64-pc-windows-gnu'
linux-target := 'x86_64-unknown-linux-gnu'

bin-name := shell("cargo metadata --format-version 1 --no-deps | jq -r '[.packages[0].targets[] | select(.kind[]) | select (.kind[] | select(. == \"bin\")) | .name][0]'")
target-dir := shell("cargo metadata --format-version 1 --no-deps | jq -r '.target_directory'")

# run the project
[arg("args", help="args to pass to the cli")]
[arg("hotpatch", long="hot", value="true", help="use hotpatching via `dx`")]
[script]
run hotpatch="false" *args:
    if {{ hotpatch }}; then
        BEVY_ASSET_ROOT='.' dx serve --hot-patch --features hotpatching {{ args }}
    else
        bevy run {{ args }}
    fi

# build the project for release
[arg("target", long="target", help="a rustc target triple, `web`, or `all` (web, linux, windows)")]
[arg("args", help="args to pass to the bevy cli")]
[script]
build target="all" *args:
    if [ '{{ target }}' == 'all' ]; then
        just build '{{ web-target }}' {{ args }}
        just build '{{ win-target }}' {{ args }}
        just build '{{ linux-target }}' {{ args }}
    elif [ '{{ target }}' == "web" ]; then
        echo "{{ BOLD }}building target '{{ target }}'{{ NORMAL }}"

        rm -rf 'target/out/{{ web-target }}'
        mkdir -p 'target/out/{{ web-target }}'

        bevy build {{ args }} --release --no-default-features web --bundle
        cp -r '{{ target-dir }}/bevy_web/web-release/{{ bin-name }}/'* 'target/out/{{ web-target }}'
    else
        echo "{{ BOLD }}building target '{{ target }}'{{ NORMAL }}"

        rm -rf 'target/out/{{ target }}'
        mkdir -p 'target/out/{{ target }}'

        ext='{{ if target =~ ".*windows.*" { ".exe" } else { "" } }}'
        bevy build {{ args }} --release --no-default-features --target '{{ target }}'
        mv '{{ target-dir }}/{{ target }}/release/{{ bin-name }}'"$ext" 'target/out/{{ target }}'
        cp -r 'assets' 'target/out/{{ target }}'
    fi

