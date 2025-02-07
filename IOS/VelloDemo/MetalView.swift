//
//  MetalView.swift
//
//  Created by LiJinlei on 2018/11/23.
//

import UIKit
import SwiftUI

class MetalViewInternal: UIView {
    override class var layerClass: AnyClass {
        return CAMetalLayer.self
    }
    
    override init(frame: CGRect) {
        super.init(frame: frame)
        configLayer()
    }
        
    required init?(coder aDecoder: NSCoder) {
        super.init(coder: aDecoder)
        configLayer()
    }
    
    private func configLayer() {
        guard let layer = self.layer as? CAMetalLayer else {
            return
        }
        self.layer.backgroundColor = UIColor.clear.cgColor

        // https://developer.apple.com/documentation/quartzcore/cametallayer/1478157-presentswithtransaction/
        layer.presentsWithTransaction = false
        layer.framebufferOnly = true
        // nativeScale is real physical pixel scale
        // https://tomisacat.xyz/tech/2017/06/17/scale-nativescale-contentsscale.html
        self.contentScaleFactor = UIScreen.main.nativeScale
    }
}

struct MetalView: UIViewControllerRepresentable {
    typealias UIViewControllerType = MetalViewController
  
    func makeUIViewController(context: Context) -> MetalViewController {
        let controller = MetalViewController()
        
        return controller
    }
    
    func updateUIViewController(_ uiViewController: MetalViewController, context: Context) {
    }
}
