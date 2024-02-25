//
//  post.swift
//  frontend
//
//  Created by Rasmus Hyldgård Samsing on 21/01/2024.
//

import Foundation

struct PopulatedPost {
    var id: Int32
    var poster: String
    var timestamp: Int64
    var image: ImageData
    var comments: [PopulatedComment]
}
