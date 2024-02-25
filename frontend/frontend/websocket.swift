//
//  websocket.swift
//  frontend
//
//  Created by Rasmus Hyldgård Samsing on 21/01/2024.
//

import Foundation


enum SocketEventType: String, Decodable, Encodable {
    case InitialState = "InitialState";
    case CommentCreated = "CommentCreated";
    case PostCreated = "PostCreated";
}

enum SocketData: Encodable, Decodable {
    case InitialState(InitialState)
    case Comment(CommentDTO)
    case Post(PostDTO)
    case CreateCommentDTO(CreateCommentDTO)
}

struct InitialState: Encodable, Decodable {
    var posts: Array<Post>
}

struct CommentDTO: Encodable, Decodable {
    var comment: Comment
}

struct PostDTO: Encodable, Decodable {
    var post: Post
}



//struct MessageVecWrapper: Encodable, Decodable {
//    var MessageVec: Array<MessageDTO>
//}

//struct MessageWrapper: Encodable, Decodable {
//    var MessagePosted: MessageDTO
//}

struct SocketResponse<T: Decodable & Encodable>: Encodable, Decodable {
    var data_type: SocketEventType;
    var data: T;
}




struct SocketResponseUnknownData: Encodable, Decodable {
    var data_type: SocketEventType;
}



func build_request() -> URLRequest {
    var user_id = UserDefaults.standard.string(forKey: "user_id")!
    var token = UserDefaults.standard.string(forKey: "token")!
    var request = URLRequest(url: URL(string:"ws://161.97.67.44:4500/websocket")!);
    request.setValue(user_id, forHTTPHeaderField: "user_id");
    request.setValue(token, forHTTPHeaderField: "token");
    return request;
}

class MessageSocket: ObservableObject {
    var socket: URLSessionWebSocketTask;
    
    
    init() {
        let request = build_request();
        print("Creating socket with user ");
        print(request.value(forHTTPHeaderField: "user_id"));
        
        
        socket = URLSession.shared.webSocketTask(with: request);
        socket.resume();
    }
    
    func receive_next_packet(on_post_posted: @escaping (Result<Post, Error>) -> Void, on_comment_posted: @escaping (Result<Comment, Error>) -> Void, on_subscribe: @escaping (Result<Array<Post>, Error>) -> Void, on_socket_error: @escaping (Error) -> Void) async -> Void{
        func completionHandler (result: URLSessionWebSocketTask.Message) {
            print("CompletionHandler called");
            print(result);
            var data_type: SocketEventType;
            var data: Data;
            
            switch result {
            case .string(let serialized_socket_data_as_string):
                do {
                    data = serialized_socket_data_as_string.data(using: String.Encoding.utf8)!;
                    print("Data: ")
                    print(data);
                    data_type = try JSONDecoder().decode(SocketResponseUnknownData.self, from: data).data_type;
                } catch {
                    print(error)
                    on_socket_error(Errors.DeserializationError("Could not deserialize outer event"))
                    return;
                }
            case _:
                on_socket_error(Errors.InvalidSocketDataFormat("Received invalid data format through socket connection."))
                return;
            
            }
            do {
                switch data_type {
                case .InitialState:
                    let resp_deserialized = try JSONDecoder().decode(SocketResponse<InitialState>.self, from: data);
                    on_subscribe(.success(resp_deserialized.data.posts))
                    return;
//                case .MessagePosted:
//                    let resp_deserialized = try JSONDecoder().decode(SocketResponse<MessageWrapper>.self, from: data);
//                    on_message_posted(.success(resp_deserialized.data.MessagePosted))
//                    return;
                    
//                case .PostMessage:
//                    return;
                case .CommentCreated:
                    let resp_deserialized = try JSONDecoder().decode(SocketResponse<CommentDTO>.self, from: data);
                    on_comment_posted(.success(resp_deserialized.data.comment))
                    return;
                case .PostCreated:
                    let resp_deserialized = try JSONDecoder().decode(SocketResponse<PostDTO>.self, from: data);
                    on_post_posted(.success(resp_deserialized.data.post))
                    return;
                }
            } catch {
                print(error)
                print("Failed deserialization")
            }
        }
        
        while true {
            print("Calling .receive...")
            do {
                let message = try await self.socket.receive();
                completionHandler(result: message);
                
            } catch {
                print("Failed when getting packet")
                print(error)
            }
            
            print("Finished call.")
        }
    }
    
//    func post_message(message: String, completionHandler: @escaping (Error?) -> Void) {
//        print("Sending message...");
//        print(message);
////        let request = build_request();
//        do {
//            let socket_resp = SocketResponse(data_type: SocketEventType.PostMessage, data: SocketData.NewMessage(message:message));
//            
//            let serialized_request = try JSONEncoder().encode(socket_resp);
//            let string_to_send = String(decoding: serialized_request, as: UTF8.self);
//            print("Sending string:");
//            print(string_to_send);
//            print("Data to send:")
//            print(serialized_request);
//            self.socket.send(URLSessionWebSocketTask.Message.string(string_to_send), completionHandler: completionHandler);
//            print("Message has been sent")
//        } catch {
//            completionHandler(Errors.SerializationError("Could not serialize data."))
//            return;
//        }
//        
//    }
}


struct CreateCommentDTO: Decodable, Encodable {
    var data: String
    var post_id: Int32
}
