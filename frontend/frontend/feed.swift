//
//  feed.swift
//  frontend
//
//  Created by Rasmus Hyldgård Samsing on 21/01/2024.
//

import SwiftUI

struct feed: View {
    @Binding var posts: [PopulatedPost]

    var body: some View {
        VStack(content: {
            ForEach($posts, id: \.self.id) { $post in
                PostView(post: $post)
            }
        })
    }
}

func get_mocked_posts() -> [PopulatedPost] {
    let user_0_post = PopulatedPost(id: 0, poster: "Rasmus", timestamp: 0, image: ImageData(id: 0, timestamp: 0, data: UIImage(named: "Code")!.pngData()!), comments: [PopulatedComment(id: 0, poster: "Jonathan", timestamp: 0, data: "hejsa fra user 0")])
    let user_1_post = PopulatedPost(id: 1, poster: "Jonathan", timestamp: 0, image: ImageData(id: 1, timestamp: 0, data: UIImage(named: "Code")!.pngData()!), comments: [PopulatedComment(id: 1, poster: "Rasmus", timestamp: 0, data: "hejsa fra user 1")])

    return [user_0_post, user_1_post]
}

struct PreviewView: View {
    @State var posts = get_mocked_posts()
    var body: some View {
        feed(posts: $posts)
    }
}

#Preview {
    PreviewView()
}

// .task {
//    print("starting socket task")
//
//    await socket_conn.receive_next_packet(
//        on_post_posted: { result in
//            switch result {
//            case .success(let post):
//                posts.append(post)
//            case .failure(let failure):
//                print(failure)
//            }
//        },
//        on_comment_posted: { _ in
//            print("event not implemented yet")
//        }, on_subscribe: { result in
//            switch result {
//            case .success(let posts):
//                self.posts = posts
//            case .failure(let error):
//                print(error)
//            }
//
//        }, on_socket_error: { error in
//            print(error)
//        })
// }
