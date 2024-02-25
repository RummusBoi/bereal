//
//  image.swift
//  frontend
//
//  Created by Rasmus Hyldgård Samsing on 21/01/2024.
//

import Foundation

struct ImageData: Decodable, Encodable {
    var id: Int32
    var timestamp: Int64
    var data: Data
}
