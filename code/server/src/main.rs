use echo::echo::echo_server::{Echo, EchoServer};
use echo::echo::{EchoRequest, EchoResponse};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Request, Response, Status, Streaming};
use tonic::transport::Server;



#[derive(Debug)]
struct EchoService;

#[tonic::async_trait]
impl Echo for EchoService {
    async fn unary_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> std::result::Result<Response<EchoResponse>, Status> {
        let message =
            request.get_ref().message.clone();
        Ok(Response::new(EchoResponse {
            message: message,
        }))
    }

    type ServerStreamingEchoStream = ReceiverStream<Result<EchoResponse, Status>>;

    async fn server_streaming_echo(
        &self,
        request: Request<EchoRequest>,
    ) -> std::result::Result<Response<Self::ServerStreamingEchoStream>, Status> {
        let (tx, rx) = mpsc::channel(1);

        tokio::spawn(async move {
            for i in 0..5 {
                let message = format!("{}: {}", i, request.get_ref().message);
                let resp = EchoResponse {
                    message: message,
                };
                tx.send(Ok(resp)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn client_streaming_echo(
        &self,
        request: Request<Streaming<EchoRequest>>,
    ) -> std::result::Result<Response<EchoResponse>, Status> {
        let mut stream = request.into_inner();
        let mut all_messages = Vec::new();
        while let Some(message)= stream.message().await? {
            all_messages.push(message.message);
        }

        Ok(Response::new(EchoResponse {
            message: all_messages.join("\n"),
        }))
    }

    type BidirectionalStreamingEchoStream = Pin<Box<dyn Stream<Item = std::result::Result<EchoResponse, Status>> + Send>>;

    async fn bidirectional_streaming_echo(
        &self,
        request: Request<Streaming<EchoRequest>>,
    ) -> std::result::Result<Response<Self::BidirectionalStreamingEchoStream>, Status> {
        let mut stream = request.into_inner();
        let output = async_stream::try_stream! {
            while let Some(message) = stream.message().await? {
                let message = message.message;
                yield EchoResponse {
                    message: message,
                };
            }
        };

        Ok(Response::new(Box::pin(output) as Self::BidirectionalStreamingEchoStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();
    let echo = EchoService {};
    let svc = EchoServer::new(echo);
    println!("Listening on {}", addr);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}


// grpcurl -plaintext -import-path=./schema/echo -proto echo.proto -d '{"message": "hello world"}' '[::1]:10000' echo.Echo/UnaryEcho
// grpcurl -plaintext -import-path=./schema/echo -proto echo.proto -d '{"message": "hello world"}' '[::1]:10000' echo.Echo/ServerStreamingEcho
// grpcurl -plaintext -import-path=./schema/echo -proto echo.proto -d '{"message": "hello"} {"message": "world"}' '[::1]:10000' echo.Echo/ClientStreamingEcho
// grpcurl -plaintext -import-path=./schema/echo -proto echo.proto -d '{"message": "hello"} {"message": "world"}' '[::1]:10000' echo.Echo/BidirectionalStreamingEcho