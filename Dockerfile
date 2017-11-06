FROM znly/protoc

RUN apk update &&\
     apk add --no-cache ca-certificates &&\
     update-ca-certificates &&\
     apk add --no-cache openssl &&\
     apk add --no-cache curl 

WORKDIR /

RUN wget $(curl -s https://api.github.com/repos/sevagh/pq/releases/latest | grep browser_download_url | cut -d '"' -f 4) -O ./pq-bin.tar.gz
RUN tar -xzf /pq-bin.tar.gz
RUN mkdir /fdsets

RUN echo 'for file in protos/*.proto; do base=$(basename "$file" .proto); protoc &>/dev/null --proto_path=/protos -o fdsets/"$base".fdset /protos/"$base".proto; done;' > /compile_protos.sh
RUN echo '/pq --fdsetdir /fdsets "${@}"' >> /compile_protos.sh
RUN chmod +x /compile_protos.sh

ENTRYPOINT ["sh", "compile_protos.sh"]
CMD ["--help"]
