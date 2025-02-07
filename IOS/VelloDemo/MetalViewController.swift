//
//  MetalViewController.swift
//
//  Created by LiJinlei on 2021/9/10.
//

import UIKit

class MetalViewController: UIViewController {
    private var metalV: MetalViewInternal!
    private var velloApp: OpaquePointer?
    
    let sceneIdx: UInt32 = 1
    
    private lazy var displayLink: CADisplayLink = {
        CADisplayLink(target: self, selector: #selector(enterFrame))
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
       
        self.displayLink.add(to: .current, forMode: .default)
        self.displayLink.isPaused = true
        
        metalV = MetalViewInternal(frame: view.frame);

        view.addSubview(metalV)
        
        self.setupWGPUCanvasIfNeeded()
    }
    
    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        
        self.view.backgroundColor = .white
        self.displayLink.isPaused = false
        
        
        let affine = view.layer.affineTransform();
        
        let frame = view.layer.frame;

        let bounds = Rectangle(
            x: Float(frame.minX),
            y: Float(frame.minY),
            width: Float(frame.width),
            height: Float(frame.height)
        );

        let scaleFactor = UIScreen.main.scale

        App_render(self.velloApp,
                   self.sceneIdx,
                   bounds,
                   Float(scaleFactor),
                   Affine(a: Float(affine.a),
                          b:Float(affine.b),
                          c:Float(affine.c),
                          d:Float(affine.d),
                          tx:Float(affine.tx),
                          ty:Float(affine.ty)
                  )
        )
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        self.displayLink.isPaused = true
    }
    
    @objc private func enterFrame() {
        guard self.velloApp == nil else { return }

     
    }
    
    private func setupWGPUCanvasIfNeeded() {
        guard self.velloApp == nil else { return }
        
        let viewPointer = Unmanaged.passUnretained(self.metalV).toOpaque()

        guard let layer = self.metalV.layer as? CAMetalLayer else {
            return
        }
        
        print("* am I framebufferOnly enabled?", layer.framebufferOnly)

        let metalLayer = Unmanaged.passUnretained(layer).toOpaque()
        let maximumFrames = Int32(UIScreen.main.maximumFramesPerSecond)
        
        let viewObj = IOSViewObj(
            view: viewPointer,
            metal_layer: metalLayer,
            maximum_frames: maximumFrames,
            callback_to_swift: callback_to_swift
        )
        
        let assetsDir = Bundle.main.resourceURL!
        
        self.velloApp = assetsDir.path.withCString{ assetsDir in
            return App_create(viewObj, assetsDir)
        }
    }
}

func callback_to_swift(arg: Int32) {
    DispatchQueue.main.async {
        switch arg {
        case 0:
            print("wgpu canvas created!")
        case 1:
            print("canvas enter frame")
        default:
            break
        }
    }
}
