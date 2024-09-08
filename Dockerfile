FROM python:3.12.5-alpine3.20
RUN pip install -U prefect
RUN prefect config set PREFECT_SERVER_API_HOST=0.0.0.0
RUN prefect config set PREFECT_API_URL=http://0.0.0.0:4200/api
EXPOSE 4200
ENTRYPOINT [ "prefect", "server", "start" ]
