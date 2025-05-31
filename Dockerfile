FROM docker.io/gentoo/portage AS portage

FROM docker.io/gentoo/stage3:musl AS builder-stage1
COPY --from=portage /var/db/repos/gentoo /var/db/repos/gentoo
RUN emerge --update --newuse --deep @world
RUN emerge app-eselect/eselect-repository
RUN emerge sys-apps/busybox
RUN echo '>=app-misc/mime-types-2.1.54 nginx' > /etc/portage/package.use/nginx
RUN emerge nginx
RUN echo 'dev-lang/rust rust-analyzer rust-src rustfmt verify-sig wasm llvm_targets_WebAssembly' > /etc/portage/package.use/rust
RUN emerge llvm-core/clang
RUN emerge rust
RUN quickpkg "*/*"

# Base system install
RUN ROOT=/gentoo USE=build emerge -1 baselayout
RUN ROOT=/gentoo emerge -K sys-libs/musl
RUN ROOT=/gentoo emerge -K sys-apps/busybox
RUN ROOT=/gentoo emerge app-misc/ca-certificates
RUN ROOT=/gentoo emerge -K nginx

# App requirements
RUN ROOT=/gentoo emerge -K gcc

# N/A

RUN cd /gentoo/usr/bin && ln -s busybox sh

FROM builder-stage1 AS passwd
RUN groupadd -g 1000 sfmbasisfaker
RUN useradd -u 1000 -g 1000 sfmbasisfaker

# Build app
FROM builder-stage1 AS builder-stage2

RUN ROOT=/gentoo emerge app-misc/ca-certificates
RUN ROOT=/gentoo emerge -K llvm-core/clang
RUN ROOT=/gentoo emerge -K rust
RUN ROOT=/gentoo emerge -K openssl
#RUN ROOT=/gentoo emerge "<sys-kernel/linux-headers-5.15"

FROM scratch AS builder-stage3
COPY --from=builder-stage2 /gentoo /

RUN OPENSSL_DIR=/usr cargo install dioxus-cli

# App install
COPY . /src
WORKDIR /src
RUN /root/.cargo/bin/dx bundle --platform web
WORKDIR /src/target/dx/sfmbasisfakerfront/release/web/public
RUN sed -e 's/\/.\//.\//g' index.html > index.html.new
RUN mv -v index.html.new index.html

FROM scratch AS builder-stage4
COPY --from=builder-stage1 /gentoo /

# Cheating
RUN rm -rfv /usr/lib/gcc/*/*/*san*
RUN rm -rfv /usr/lib/gcc/*/*/*.a
RUN rm -rfv /usr/lib/gcc/*/*/include
RUN rm -rfv /usr/lib/gcc/*/*/plugin
RUN rm -rfv /usr/include
RUN rm -rfv /usr/libexec
RUN rm -rfv /usr/share/doc
RUN rm -rfv /usr/x86_64-pc-linux-musl
RUN rm -rfv /var/db/pkg
#RUN mv -v /usr/lib64/* /usr/lib

# Bragging
RUN ls /

# Cleanup
FROM scratch
COPY --from=builder-stage4 / /
COPY --from=builder-stage3 /src/target/dx/sfmbasisfakerfront/release/web/public /sfmbasisfakerfront
COPY --from=passwd /etc/passwd /etc/passwd
COPY --from=passwd /etc/group /etc/group
COPY --from=passwd /etc/shadow /etc/shadow
COPY --from=passwd /home/sfmbasisfaker /home/sfmbasisfaker
COPY ./container/nginx.conf /etc/nginx/nginx.conf
RUN chown sfmbasisfaker:sfmbasisfaker /var/log/nginx
RUN mkdir -p /var/lib/nginx/tmp
RUN chown sfmbasisfaker:sfmbasisfaker /var/lib/nginx/tmp
RUN mkdir -p /run/nginx
RUN chown sfmbasisfaker:sfmbasisfaker /run/nginx
RUN ls /sfmbasisfakerfront
USER sfmbasisfaker
EXPOSE 8080
ENTRYPOINT [ "/usr/bin/nginx" ]
CMD [ "-c", "/etc/nginx/nginx.conf" ]
