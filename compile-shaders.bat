REM TODO: Real shader compilation pipeline

C:/VulkanSDK/1.4.304.0/Bin/glslc.exe shaders/src/shader-base.frag -o shaders/spv/shader-base-frag.spv
C:/VulkanSDK/1.4.304.0/Bin/glslc.exe shaders/src/shader-base.vert -o shaders/spv/shader-base-vert.spv
C:/VulkanSDK/1.4.304.0/Bin/glslc.exe shaders/src/shader-depth.frag -o shaders/spv/shader-depth-frag.spv
C:/VulkanSDK/1.4.304.0/Bin/glslc.exe shaders/src/shader-depth.vert -o shaders/spv/shader-depth-vert.spv