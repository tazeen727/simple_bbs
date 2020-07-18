FROM node:12-alpine

WORKDIR /app
RUN npm install -g sass
COPY ./docker-entrypoint.sh /usr/local/bin
RUN chmod u+x /usr/local/bin/docker-entrypoint.sh

EXPOSE 8080
ENTRYPOINT ["sh", "-c", "/usr/local/bin/docker-entrypoint.sh"]
CMD []
