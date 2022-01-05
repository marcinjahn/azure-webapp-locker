var axios = require("axios");
var qs = require("qs");

const priorityNumber = 444;
const resourceUrl =
  "https://management.azure.com/subscriptions/152b9fab-23f1-4afd-9048-dd45885ab0c4/resourceGroups/Ability-Sandbox/providers/Microsoft.Web/sites/ability-sandbox-proxy/config/web?api-version=2018-11-01";

main();

async function main() {
  const param = process.argv[process.argv.length - 1]
  if (param === "1") {
    await enable();
  }
  else if(param === "0")
    await disable();
  else
    console.log("0 or 1 needs to be provided to enable or disable the access");
}

async function enable() {
  const accessToken = await getAccessToken();
  const currentConfig = await getCurrentConfig(accessToken);
  if (currentConfig.some((n) => n.priority === priorityNumber)) {
    console.log(`The rule ${priorityNumber} already exists. Ignoring`);
    return;
  }
  currentConfig.push(createAllowRule());
  await patchRules(accessToken, currentConfig);
}

async function disable() {
  const accessToken = await getAccessToken();
  const currentConfig = await getCurrentConfig(accessToken);
  const index = currentConfig.findIndex((n) => n.priority == priorityNumber);
  if (index === -1) {
    console.log("The rule does not exist. Ignoring");
    return;
  }
  currentConfig.splice(index, 1);
  await patchRules(accessToken, currentConfig);
}

function createAllowRule() {
  return {
    ipAddress: "0.0.0.0/0",
    action: "Allow",
    tag: "Default",
    priority: priorityNumber,
    name: "Temp Allow",
  };
}

async function patchRules(accessToken, ipSecurityRestrictions) {
  const data = JSON.stringify({
    id: "/subscriptions/152b9fab-23f1-4afd-9048-dd45885ab0c4/resourceGroups/Ability-Sandbox/providers/Microsoft.Web/sites/ability-sandbox-proxy/config/web",
    name: "ability-sandbox-proxy",
    type: "Microsoft.Web/sites/config",
    location: "North Europe",
    properties: {
      ipSecurityRestrictions,
    },
  });

  const config = {
    method: "patch",
    url: resourceUrl,
    headers: {
      Authorization: "Bearer " + accessToken,
      "Content-Type": "application/json",
    },
    data,
  };

  const result = await axios(config);
  if (result.status >= 400) {
    throw new Error("PATCH failed");
  }
}

async function getCurrentConfig(accessToken) {
  const config = {
    method: "get",
    url: resourceUrl,
    headers: {
      Authorization: "Bearer " + accessToken,
    },
  };

  const result = await axios(config);
  return result.data.properties.ipSecurityRestrictions;
}

async function getAccessToken() {
  const clientId = "fa8177f4-d1c9-4d2e-9cf3-66f0625cd693";
  const secret = "r2U7Q~HGnYHx0b2hTTCJISKOGU-AHRvVxx~F0";

  const data = qs.stringify({
    resource: "https://management.azure.com/",
    client_id: clientId,
    client_secret: secret,
    grant_type: "client_credentials",
  });

  const config = {
    method: "post",
    url: "https://login.microsoftonline.com/372ee9e0-9ce0-4033-a64a-c07073a91ecd/oauth2/token",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
    data: data,
  };

  const result = await axios(config);
  return result.data.access_token;
}
