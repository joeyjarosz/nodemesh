use protocol::messages;
use transport::Transport;

pub trait FromResponse {
    fn from_response(response: &messages::Response) -> Self;
}

pub struct GetVersionResult {
    pub version: String
}

impl FromResponse for GetVersionResult {
    fn from_response(response: &messages::Response) -> Self {
        GetVersionResult{
            version: response.get_get_version_response().get_version().to_string()
        }
    }
}

pub struct PipelineResult {
    responses: messages::Responses
}

impl PipelineResult {
    pub fn get<T>(&self, index: usize) -> T
        where T: FromResponse
    {
        let responses = self.responses.get_responses();
        FromResponse::from_response(&responses[index])
    }
}

pub struct Pipeline {
    requests: messages::Requests
}

impl Pipeline {
    fn new() -> Pipeline {
        Pipeline{
            requests: messages::Requests::new()
        }
    }

    pub fn get_version(&mut self) -> &mut Pipeline {
        let mut request = messages::Request::new();
        request.set_request_type(
            messages::Request_RequestType::GET_VERSION
        );
        self.requests.mut_requests().push(request);
        self
    }

    pub fn send(&self, client: &mut Client) -> Result<PipelineResult, String> {
        let mut transport = client.transport.as_mut();
        let responses = try!(transport.send(&self.requests));
        Ok(PipelineResult{responses: responses})
    }
}

pub fn pipe() -> Pipeline {
    Pipeline::new()
}

pub struct Client {
    transport: Box<Transport>
}

impl Client {
    pub fn new(transport: Box<Transport>) -> Client {
        Client{transport: transport}
    }

    pub fn get_version(&mut self) -> Result<String, String> {
        let mut pipeline = pipe();
        pipeline.get_version().get_version();
        let result = try!(pipeline.send(self));
        let result: GetVersionResult = result.get(0);
        Ok(result.version)
    }
}
