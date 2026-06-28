// ─── Three.js 3D Particle System ───
(function initThree() {
  const canvas = document.getElementById('bg-canvas');
  if (!canvas) return;

  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
  const renderer = new THREE.WebGLRenderer({
    canvas,
    alpha: true,
    antialias: true,
  });
  renderer.setSize(window.innerWidth, window.innerHeight);
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));

  // ── Stars ──
  const starCount = 2000;
  const starsGeo = new THREE.BufferGeometry();
  const positions = new Float32Array(starCount * 3);
  const colors = new Float32Array(starCount * 3);
  const sizes = new Float32Array(starCount);

  for (let i = 0; i < starCount; i++) {
    positions[i * 3] = (Math.random() - 0.5) * 100;
    positions[i * 3 + 1] = (Math.random() - 0.5) * 100;
    positions[i * 3 + 2] = (Math.random() - 0.5) * 100;

    const c = new THREE.Color();
    c.setHSL(0.08 + Math.random() * 0.1, 0.8, 0.4 + Math.random() * 0.5);
    colors[i * 3] = c.r;
    colors[i * 3 + 1] = c.g;
    colors[i * 3 + 2] = c.b;

    sizes[i] = 0.02 + Math.random() * 0.04;
  }

  starsGeo.setAttribute('position', new THREE.BufferAttribute(positions, 3));
  starsGeo.setAttribute('color', new THREE.BufferAttribute(colors, 3));
  starsGeo.setAttribute('size', new THREE.BufferAttribute(sizes, 1));

  const starMat = new THREE.PointsMaterial({
    size: 0.04,
    vertexColors: true,
    transparent: true,
    opacity: 0.8,
    blending: THREE.AdditiveBlending,
    depthWrite: false,
    sizeAttenuation: true,
  });

  const stars = new THREE.Points(starsGeo, starMat);
  scene.add(stars);

  // ── Orbiting Particles ──
  const orbCount = 400;
  const orbGeo = new THREE.BufferGeometry();
  const orbPos = new Float32Array(orbCount * 3);
  const orbColors = new Float32Array(orbCount * 3);
  const orbSizes = new Float32Array(orbCount);
  const orbData = [];

  for (let i = 0; i < orbCount; i++) {
    const theta = Math.random() * Math.PI * 2;
    const phi = Math.acos(2 * Math.random() - 1);
    const radius = 5 + Math.random() * 25;

    orbPos[i * 3] = radius * Math.sin(phi) * Math.cos(theta);
    orbPos[i * 3 + 1] = radius * Math.sin(phi) * Math.sin(theta);
    orbPos[i * 3 + 2] = radius * Math.cos(phi);

    const hue = 0.07 + Math.random() * 0.15;
    const c = new THREE.Color().setHSL(hue, 0.9, 0.5 + Math.random() * 0.3);
    orbColors[i * 3] = c.r;
    orbColors[i * 3 + 1] = c.g;
    orbColors[i * 3 + 2] = c.b;

    orbSizes[i] = 0.04 + Math.random() * 0.08;

    orbData.push({
      theta, phi, radius,
      speed: 0.0005 + Math.random() * 0.002,
      phase: Math.random() * Math.PI * 2,
    });
  }

  orbGeo.setAttribute('position', new THREE.BufferAttribute(orbPos, 3));
  orbGeo.setAttribute('color', new THREE.BufferAttribute(orbColors, 3));
  orbGeo.setAttribute('size', new THREE.BufferAttribute(orbSizes, 1));

  const orbMat = new THREE.PointsMaterial({
    size: 0.06,
    vertexColors: true,
    transparent: true,
    opacity: 0.6,
    blending: THREE.AdditiveBlending,
    depthWrite: false,
    sizeAttenuation: true,
  });

  const orbs = new THREE.Points(orbGeo, orbMat);
  scene.add(orbs);

  // ── Connecting lines ──
  const lineMat = new THREE.LineBasicMaterial({
    color: 0xF97316,
    transparent: true,
    opacity: 0.03,
  });
  const linePositions = new Float32Array(600);
  const lineGeo = new THREE.BufferGeometry();
  lineGeo.setAttribute('position', new THREE.BufferAttribute(linePositions, 3));
  const lines = new THREE.LineSegments(lineGeo, lineMat);
  scene.add(lines);

  let lineDirty = true;

  camera.position.z = 20;

  // ── Mouse tracking ──
  let mouseX = 0, mouseY = 0;
  document.addEventListener('mousemove', (e) => {
    mouseX = (e.clientX / window.innerWidth) * 2 - 1;
    mouseY = (e.clientY / window.innerHeight) * 2 - 1;
  });

  // ── Resize ──
  window.addEventListener('resize', () => {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
  });

  // ── Animation loop ──
  function animate(t) {
    requestAnimationFrame(animate);

    // Rotate star field slowly
    stars.rotation.y += 0.0001;
    stars.rotation.x += 0.00005;

    // Update orbiting particles
    const orbPosAttr = orbGeo.attributes.position;
    const array = orbPosAttr.array;

    for (let i = 0; i < orbCount; i++) {
      const d = orbData[i];
      const theta = d.theta + t * d.speed;
      const phi = d.phi + t * d.speed * 0.3;

      array[i * 3] = d.radius * Math.sin(phi) * Math.cos(theta);
      array[i * 3 + 1] = d.radius * Math.sin(phi) * Math.sin(theta);
      array[i * 3 + 2] = d.radius * Math.cos(phi);
    }
    orbPosAttr.needsUpdate = true;

    // Update connecting lines (every 60 frames)
    if (lineDirty || Math.floor(t / 16) % 60 === 0) {
      const pos = lineGeo.attributes.position.array;
      let idx = 0;
      for (let i = 0; i < Math.min(orbCount, 100); i += 2) {
        if (idx + 5 < pos.length) {
          const idx1 = i * 3;
          const idx2 = (i + 1) * 3;
          pos[idx++] = orbPosAttr.array[idx1];
          pos[idx++] = orbPosAttr.array[idx1 + 1];
          pos[idx++] = orbPosAttr.array[idx1 + 2];
          pos[idx++] = orbPosAttr.array[idx2];
          pos[idx++] = orbPosAttr.array[idx2 + 1];
          pos[idx++] = orbPosAttr.array[idx2 + 2];
        }
      }
      lineGeo.attributes.position.needsUpdate = true;
      lineDirty = false;
    }

    // Mouse parallax on camera
    const targetX = mouseX * 2;
    const targetY = mouseY * 1.5;
    camera.position.x += (targetX - camera.position.x) * 0.02;
    camera.position.y += (-targetY - camera.position.y) * 0.02;
    camera.lookAt(0, 0, 0);

    renderer.render(scene, camera);
  }

  animate(0);
})();

// ─── Scroll Reveal ───
(function setupReveal() {
  const els = document.querySelectorAll('[data-reveal]');

  const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        entry.target.classList.add('revealed');
        observer.unobserve(entry.target);
      }
    });
  }, {
    threshold: 0.15,
    rootMargin: '0px 0px -50px 0px',
  });

  els.forEach((el) => observer.observe(el));
})();

// ─── Mouse Parallax Tilt ───
(function setupTilt() {
  const cards = document.querySelectorAll('[data-tilt]');

  cards.forEach((card) => {
    card.addEventListener('mousemove', (e) => {
      const rect = card.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;
      const cx = rect.width / 2;
      const cy = rect.height / 2;
      const dx = (x - cx) / cx;
      const dy = (y - cy) / cy;
      const rotateX = dy * -8;
      const rotateY = dx * 8;
      card.style.transform = `perspective(800px) rotateX(${rotateX}deg) rotateY(${rotateY}deg) scale3d(1.02,1.02,1.02)`;
    });

    card.addEventListener('mouseleave', () => {
      card.style.transform = 'perspective(800px) rotateX(0deg) rotateY(0deg) scale3d(1,1,1)';
    });
  });
})();

// ─── Navbar scroll effect ───
(function setupNavbar() {
  const navbar = document.getElementById('navbar');
  window.addEventListener('scroll', () => {
    navbar.classList.toggle('scrolled', window.scrollY > 50);
  });
})();

// ─── Theme tabs ───
(function setupThemeTabs() {
  const tabs = document.querySelectorAll('.theme-tab');
  const body = document.getElementById('preview-body');

  tabs.forEach((tab) => {
    tab.addEventListener('click', () => {
      tabs.forEach((t) => t.classList.remove('active'));
      tab.classList.add('active');

      if (tab.dataset.theme === 'neon') {
        body.className = 'preview-body neon-theme';
      } else {
        body.className = 'preview-body gradient-theme';
      }
    });
  });
})();

// ─── Hamburger menu ───
(function setupHamburger() {
  const btn = document.getElementById('hamburger');
  const links = document.querySelector('.nav-links');
  btn.addEventListener('click', () => {
    links.classList.toggle('open');
    btn.classList.toggle('open');
  });
})();
