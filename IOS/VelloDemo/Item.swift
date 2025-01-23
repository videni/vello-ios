//
//  Item.swift
//  Vello-IOS-Demo
//
//  Created by vidy videni on 2025/1/22.
//

import Foundation
import SwiftData

@Model
final class Item {
    var timestamp: Date
    
    init(timestamp: Date) {
        self.timestamp = timestamp
    }
}
