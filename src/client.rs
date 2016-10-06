use protobuf;

use protocol::messages;
use transport::Transport;

use node::Node;
use types::Id;

fn get_node_message(node: &Box<Node>) -> messages::Node {
    let mut message = messages::Node::new();
    message.set_node_type(node.get_type());
    let children = protobuf::RepeatedField::from_vec(
        node.as_ref().get_state().children.iter().map(get_node_message).collect()
    );
    message.set_children(children);
    message
}

fn get_node_messages(
    nodes: Vec<Box<Node>>
) -> protobuf::RepeatedField<messages::Node> {
    protobuf::RepeatedField::from_vec(
        nodes.iter().map(get_node_message).collect()
    )
}

pub struct Graph {
    id: Id
}

pub trait FromResponse {
    fn from_response(response: &messages::Response) -> Self;
}

pub struct GetVersionResult {
    pub version: String
}

impl FromResponse for GetVersionResult {
    fn from_response(response: &messages::Response) -> Self {
        GetVersionResult{
            version: response
                .get_get_version_response()
                .get_version()
                .to_string()
        }
    }
}

pub struct AddGraphResult {
    pub graph: Graph
}

impl FromResponse for AddGraphResult {
    fn from_response(response: &messages::Response) -> Self {
        AddGraphResult{
            graph: Graph{
                id: response.get_index(),
                // id: response.get_state_added_result().get_id()
            }
        }
    }
}

pub struct PipelineResult {
    // TODO: match the responses with the requests using response.index
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

pub struct GraphPipeline<'a> {
    graph: Option<Graph>,
    pipeline: &'a mut Pipeline
}

impl<'a> GraphPipeline<'a> {
    pub fn insert_nodes(
        &'a mut self,
        parent: Option<&Node>,
        previous_sibling: Option<&Node>,
        nodes: Vec<Box<Node>>
    ) -> &mut GraphPipeline {
        let mut request = messages::Request::new();
        request.set_request_type(
            messages::Request_RequestType::ADD_NODES_REQUEST
        );
        let mut message = messages::AddNodesRequest::new();

        if let Some(ref graph) = self.graph {
            message.set_graph_id(graph.id);
        }
        if let Some(parent) = parent {
            message.set_parent_id(parent.get_state().id);
        }
        if let Some(previous_sibling) = previous_sibling {
            message.set_previous_sibling_id(previous_sibling.get_state().id);
        }
        message.set_nodes(get_node_messages(nodes));

        request.set_add_nodes_request(message);
        self.pipeline.requests.mut_requests().push(request);
        self
    }

    pub fn remove_nodes(&'a mut self, nodes: Vec<&mut Node>) -> &mut GraphPipeline {
        let mut request = messages::Request::new();
        request.set_request_type(
            messages::Request_RequestType::REMOVE_NODES_REQUEST
        );
        let mut message = messages::RemoveNodesRequest::new();

        if let Some(ref graph) = self.graph {
            message.set_graph_id(graph.id);
        }
        message.set_node_ids(nodes.iter().map(|node| node.get_state().id).collect());

        request.set_remove_nodes_request(message);
        self.pipeline.requests.mut_requests().push(request);
        self
    }

    // fn apply_changes<T>(
    //     &mut self,
    //     nodes: Vec<Node + T>
    // ) -> GraphPipeline {

    // }

    // fn end(&mut self) -> &mut Pipeline {
    //     self.pipeline
    // }
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

    pub fn add_graph(&mut self, nodes: Vec<Box<Node>>) -> GraphPipeline {
        let mut request = messages::Request::new();
        request.set_request_type(
            messages::Request_RequestType::ADD_GRAPH_REQUEST
        );
        let mut message = messages::AddGraphRequest::new();
        message.set_nodes(get_node_messages(nodes));

        request.set_add_graph_request(message);
        self.requests.mut_requests().push(request);
        GraphPipeline{pipeline: self, graph: None}
    }

    pub fn use_graph(&mut self, graph: Graph) -> GraphPipeline {
        let mut request = messages::Request::new();
        request.set_request_type(
            messages::Request_RequestType::USE_GRAPH_REQUEST
        );
        let mut message = messages::UseGraphRequest::new();
        message.set_graph_id(graph.id);

        request.set_use_graph_request(message);
        self.requests.mut_requests().push(request);
        GraphPipeline{pipeline: self, graph: Some(graph)}
    }

    pub fn remove_graph(&mut self, graph: Graph) -> &mut Pipeline {
        let mut request = messages::Request::new();
        request.set_request_type(
            messages::Request_RequestType::REMOVE_GRAPH_REQUEST
        );
        let mut message = messages::RemoveGraphRequest::new();
        message.set_graph_id(graph.id);

        request.set_remove_graph_request(message);
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
        pipeline.get_version();
        let result = try!(pipeline.send(self));
        let result: GetVersionResult = result.get(0);
        Ok(result.version)
    }

    pub fn add_graph(
        &mut self, nodes: Vec<Box<Node>>
    ) -> Result<Graph, String> {
        let mut pipeline = pipe();
        pipeline.add_graph(nodes);
        let result = try!(pipeline.send(self));
        let result: AddGraphResult = result.get(0);
        Ok(result.graph)
    }
}
