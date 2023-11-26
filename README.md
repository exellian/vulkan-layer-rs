# vulkan-layer-rs
A template for creating a vulkan layer with rust. This can be used to create your own windowing system and support vulkan as a Graphics API.
The whole template is based on the guide: https://renderdoc.org/vulkan-layer-guide.html

## Layers in vulkan

Layers in vulkan are actually pretty simple. A layer is simply an interceptor of every vulkan call e.g: (vkCreateInstance, vkCreateDevice, etc.).
Every vulkan call? Yes every vulkan call, but for calls that the layer itself won't handle it simply has to forward these calls to the next layer.
In the layer you can then do what ever you want. You only have to assure that you always also call the same functions on the next layer. 
A layer itself is simply a dynamic-library (.dll, .dylib, .so) that is loaded by the vulkan-loader. But 
only if you add a .json file in a specific folder where the vulkan-loader can find it (set environment-variable: VK_ADD_LAYER_PATH or VK_LAYER_PATH).
Inside the json file there is a library_path-field which points to the actual path of the dynamic-lib (look into wsi/build.rs for more details).

## WIP (windowing sub system)

Currently, there also some modules which are unused at the moment (swapchain.rs, surface.rs). 
These should demonstrate how you can integrate your own windowing system in the future. But work is still in progress

That's basically it, for more details look in the code or in the provided guide which i used to create this project. Thank you and have a nice morning, day, evening or night!


