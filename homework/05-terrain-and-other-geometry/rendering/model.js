const IlliniOrange = new Float32Array([1, 0.373, 0.02, 1]);

const renderModel = async ({ model, lightPosX, lightPosY, lightPosZ, ...others }) => {
  const modelPath = 'models/' + model;
  const modelData = await fetch(modelPath).then(m => m.json());
  return [modelData, { ...others, lightDir: normalize([lightPosX,lightPosY,lightPosZ]), model: m4mul(m4rotX(-Math.PI/2), m4scale(.2, .2, .2)), color: IlliniOrange }];
};

export default renderModel;
