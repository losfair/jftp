FROM alpine:latest

COPY ./jftp /

EXPOSE 3000/tcp
ENTRYPOINT /jftp -l 0.0.0.0:3000
