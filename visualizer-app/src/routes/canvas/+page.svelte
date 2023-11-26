<script>
  import { onMount, afterUpdate, onDestroy } from 'svelte';

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

  const resizeCanvas = () => {
    canvas.width = window.innerWidth * 0.9; // Set canvas width to 90% of window width
    canvas.height = (window.innerHeight * 1.0) - bodyMarginHeight - 19.2; // Set canvas height to 80% of window height with 10px offset
    circle.x = 0;
    circle.y = canvas.height / 2;
    circle.radius = canvas.height / 20;
  };

  onMount(() => {
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

  function draw() {
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
</div>

<style>
  .canvas-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
  }

  canvas {
    border: 1px solid black;
    width: 90%;
    height: 100%;
  }
</style>

