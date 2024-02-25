//
//  post.swift
//  frontend
//
//  Created by Rasmus Hyldgård Samsing on 21/01/2024.
//
import SwiftUI
import UIKit

struct PostView: View {
    @Binding var post: PopulatedPost

    var body: some View {
        VStack {
            HStack(content: {
                VStack(content: {
                    HStack(content: {
                        Text(String(post.poster))
                        Text(String(post.timestamp))
                    })
                    HStack(content: {
                        Image(uiImage: data_to_image(data: post.image.data)).resizable().aspectRatio(contentMode: .fit)
                    })
                    // Spacer()
                })
            }).frame(alignment: .leading)
        }
    }
}

func get_mocked_post() -> PopulatedPost {
    let bytes: [UInt8] = (0 ..< 10000).indices.map { _ in 50 }.map { $0 as UInt8 }
//    let image = bytes_to_image(bytes: bytes)
    return PopulatedPost(id: 0, poster: "Rasmus Samsing", timestamp: 0, image: ImageData(id: 0, timestamp: 0, data: get_mock_image_data()), comments: [PopulatedComment(id: 0, poster: "Jonathan Samsing", timestamp: 0, data: "Hej du der")])
}

struct PostPreview: View {
    @State var post = get_mocked_post()
    var body: some View {
        PostView(post: $post)
    }
}

#Preview {
    PostPreview()
}

func get_mock_image_data() -> Data {
    let img = UIImage(named: "Code")!
    return img.pngData()!
}

func data_to_image(data: Data) -> UIImage {
    return UIImage(data: data)!
}
