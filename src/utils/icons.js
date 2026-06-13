
const modules = import.meta.glob('../images/icons/*.svg', {
    import: 'default',
    query: '?raw',
    eager: true
});

const icons = {};

Object.keys(modules).forEach((path) => {
    const list = path.toLowerCase().split('/');
    const filename = list.pop();
    const iconName = filename.slice(0, -4);
    const svg = modules[path];
    icons[iconName] = svg;
});

// console.log(icons);

export default icons;
