<script>
    import init, { greet, start_sim,step_sim } from 'droneviz';
    import { onMount, afterUpdate, onDestroy } from 'svelte';
  
    let droneCount = 100;
    let run_sim = false;
    let canvas;
    let ctx;
    let circle = {
      x: 0,
      y: 0,
      radius: 1,
    };
  
    
  
    const animationDuration = 2000; // 10 seconds
    let animationStartTime;
    let bodyMarginHeight;
    let dronePos;
  
    const resizeCanvas = () => {
      canvas.width = window.innerWidth * 0.9; // Set canvas width to 90% of window width
      canvas.height = (window.innerHeight * 1.0) - bodyMarginHeight - 19.2; // Set canvas height to 80% of window height with 10px offset
      circle.x = 0;
      circle.y = canvas.height / 2;
      circle.radius = canvas.height / 20;
    };
  
  
  
    onMount(
      async() => {
          await init();
          dronePos = start_sim(droneCount);
          console.log(dronePos);
  
      },  
    () => {
  
      resizeCanvas();
      draw();
      window.addEventListener('resize', resizeCanvas);
  
      return () => {
        window.removeEventListener('resize', resizeCanvas);
      };
    });
  
    afterUpdate(() => {
      resizeCanvas();
      draw();
    });
  
    onDestroy(() => {
      animationStartTime = null; // Reset animation start time when the component is destroyed
    });
  

    

    function getPointOnArc(centerX, centerY, radius, angle) {
      let x = centerX + radius * Math.cos(angle);
      let y = centerY + radius * Math.sin(angle);
      
      return [x,y];
    }
  
    function drawDronePos(ctx,canvas) {

      for (const [x,y,h] of dronePos){
  
        const minCanvasSize = Math.min(canvas.height, canvas.width);
        
        let radius = minCanvasSize / 90;
        let margin = radius
        let x_offset = margin + x*(canvas.width-2*radius);
        let y_offset = margin + y*(canvas.height-2*radius);
        let heading = h+Math.PI/2;
  
        ctx.beginPath();
        ctx.arc(x_offset, y_offset, radius, 0, 2 * Math.PI);
        ctx.fillStyle = 'red';
        ctx.stroke();
        
        let pt1 = getPointOnArc(x_offset, y_offset, radius, heading-150*Math.PI/180-Math.PI/2);
        let pt2 = getPointOnArc(x_offset, y_offset, radius, heading-Math.PI/2);
        let pt3 = getPointOnArc(x_offset, y_offset, radius, heading+150*Math.PI/180-Math.PI/2);
        
        ctx.beginPath();
        ctx.moveTo(pt1[0],pt1[1]);
        ctx.lineTo(pt2[0],pt2[1]);
        ctx.lineTo(pt3[0],pt3[1]);
        ctx.closePath();
        ctx.fillStyle='blue';
        ctx.fill();
    }

    const [x,y,h] = dronePos.at(-1);
    console.log(x,y,h);
  
    const minCanvasSize = Math.min(canvas.height, canvas.width);
    
    let radius = minCanvasSize / 90;
    let margin = radius
    let x_offset = margin + x*(canvas.width-2*radius);
    let y_offset = margin + y*(canvas.height-2*radius);
    let heading = h+Math.PI/2;

    ctx.beginPath();
    ctx.arc(x_offset, y_offset, radius, 0, 2 * Math.PI);
    ctx.fillStyle = 'green';
    ctx.fill();
    

  }
  
  function handleClick(){
    run_sim = false;
    dronePos = start_sim(droneCount);
  }

  function handleClickStart(){
    run_sim = true;
  }
  
    function draw() {
        if (run_sim){
            dronePos = step_sim();
        }

      if (typeof document === 'undefined') return; // Check if running in a non-browser environment
  
      ctx = canvas?.getContext('2d');
      if (!ctx) {
        canvas = document.createElement('canvas');
        ctx = canvas.getContext('2d');
        document.querySelector('.canvas-container').appendChild(canvas);
      }
      ctx.clearRect(0, 0, canvas.width, canvas.height); // Clear the canvas before drawing
  
      const currentTime = Date.now();
      if (!animationStartTime) {
        animationStartTime = currentTime;
      }
  
      /* 
      const elapsedTime = currentTime - animationStartTime;
      const progress = elapsedTime / animationDuration;
      circle.x = progress * (canvas.width - circle.radius);
  
      if (circle.x >= canvas.width - circle.radius) {
        circle.x = 0 - 2;
        animationStartTime = null;
      }
      
  
      ctx.beginPath();
      ctx.arc(circle.x, circle.y, circle.radius, 0, 2 * Math.PI);
      ctx.fillStyle = 'blue';
      ctx.fill();
      */
      drawDronePos(ctx,canvas);
    }
  
    setInterval(() => {
      draw();
    }, 10); // Refresh every 10 milliseconds
  
    onMount(() => {
      const bodyStyles = getComputedStyle(document.body);
      bodyMarginHeight = parseFloat(bodyStyles.marginTop) + parseFloat(bodyStyles.marginBottom);
    });
  </script>
  
  <div class="canvas-container">
    <canvas bind:this={canvas}></canvas>
    <bar>
        <button on:click={handleClick} >Reset</button>
        <button on:click={handleClickStart}> Start </button>
    </bar>
    
    
  </div>
  
<style>  
    .canvas-container {
        display: flex;
        justify-content: center;
        align-items: flex-end; /* Align to the bottom */
        height: 100%;
    }

    canvas {
        border: 1px solid black;
        width: 90%;
        height: 100%;
        position: relative;
        z-index: 0;
    }
    bar {
        position: absolute;
        z-index: 1;
    }
</style>

  
