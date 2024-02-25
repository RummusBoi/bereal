//
//  errors.swift
//  frontend
//
//  Created by Rasmus Hyldgård Samsing on 21/01/2024.
//

import Foundation


enum Errors: Error {
    case MyError
    case ServerConnectionError(String)
    case SerializationError(String)
    case DeserializationError(String)
    case InvalidSocketDataFormat(String)
    
}
