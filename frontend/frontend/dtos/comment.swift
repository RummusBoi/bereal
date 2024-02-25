//
//  comment.swift
//  frontend
//
//  Created by Rasmus Hyldgård Samsing on 21/01/2024.
//

import Foundation

struct Comment: Decodable, Encodable  {
    var id: Int32
    var poster_id: Int32
    var timestamp: Int64
    var data: String
}

