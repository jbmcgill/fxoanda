import json
import textwrap
import re
from pprint import pprint
from jinja2 import Environment, FileSystemLoader

from argparse import ArgumentParser

def snake(s):
 if s == 'type':
    return 'otype';
 s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', s)
 return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()

def ucfirst(x):
 return x[0].upper() + x[1:]

def lcfirst(x):
 return x[0].lower() + x[1:]

def camel(s):
 return ucfirst(s).title().replace("_","")

def get_schema():
  with open("./v20.json","r") as f:
    schema = json.load(f)
  schema['paths']['/instruments/{instrument}/positionBook']['get']['operationId'] = 'getPositionBook'
  schema['paths']['/instruments/{instrument}/orderBook']['get']['operationId'] = 'getOrderBook'
  schema['paths']['/accounts/{accountID}/instruments/{instrument}/candles']['get']['operationId'] = 'getAccountInstrumentCandles'
  schema['paths']['/accounts/{accountID}/pricing']['get']['parameters'][3]['type'] = 'string'
  return schema

def parse_ref(s):
  ref_section, ref_name = s.split("/")[-2:]
  return (ref_section, ref_name)

def get_ref(schema, param):
  if '$ref' in param:
    ref_section, ref_name = parse_ref(param['$ref'])
    result = schema[ref_section][ref_name]
    return result
  else:
    return param

def get_args():
  parser = ArgumentParser()
  parser.add_argument("-f", "--file", help="location of v20.json", default="./v20.json")
  parser.add_argument("-t", "--type", help="type of parse (request,...)")
  parser.add_argument("-u", "--url", help="name of url structure to parse")
  parser.add_argument("-m", "--method", help="name of method structure to parse")
  parser.add_argument("-g", "--gen", help="type of structure to generate", default='struct')
  return parser.parse_args()

def get_comment(schema,o):
    result = ""
    if 'summary' in o:
        result = result + "/// {}\n".format(textwrap.fill(o['summary']).replace("\n","\n/// "))
    if 'description' in o:
        result = result + "/// {}\n".format(textwrap.fill(o['description']).replace("\n","\n/// "))
    if 'format' in o:
        result = result + "/// format: {}\n".format(textwrap.fill(o['format']).replace("\n","\n/// "))
    if '$ref' in o:
      ref_name = o['$ref'].split("/")[-1]
      for t in ['definitions', 'responses', 'parameters']:
        if ref_name in schema[t]:
          result = get_comment(schema,schema[t][ref_name])

    return result

def get_array_type(schema, param):
  item = get_ref(schema, param['items'])
  return get_param_type(schema, item)

def get_param_type(schema, param):
  result = '' 
  if 'schema' in param:
      t = get_param_type(schema,param['schema']['properties'])
  else:
      t = param['type']
  comment = get_comment(schema, param)
  if 'rfc3339' in comment:
    result = 'DateTime<Utc>'
  elif 'A decimal number' in comment:
    result = 'f32'
  elif 'name' in param and param['name'] == 'granularity':
    result = 'CandlestickGranularity'
  elif t == 'string':
    result = 'String'
  elif t in ['int','integer']:
    result = 'i32'
  elif t == 'boolean':
    result = 'bool' 
  elif t == 'array':
    result = 'array' #get_array_type(schema, param)
  elif t == 'object':
    if 'name' in param:
      result = ucfirst(param['name'].replace('-','')) 
    else:
      result = "Vec<{}>".format(t)
  else:
    result = t
  return result

def parse_param(param, schema):
  if '$ref' not in param:
    #print("parse_param (a): ", param);
    result = dict()
    result['name'] = param['name'].replace('-','')
    result['snake'] = snake(param['name'].replace('-',''))
    result['comment'] = get_comment(schema,param)
    result['in'] = param['in']
    result['type'] = get_param_type(schema, param)
    if result['type'] == 'array':
      if '$ref' in param['items']:
        ref_section, ref_name = parse_ref(param['items']['$ref'])
        result['type'] = "Vec<{}>".format(ref_name) 
      else:
        array_type = get_param_type(schema, param['items'])
        result['type'] = "Vec<{}>".format(array_type) 
    return result 
  else:
    #print("parse_param (b): ", param)
    ref_name = param['$ref'].split("/")[-1]
    if ref_name in schema['parameters']:
        return parse_param( schema['parameters'][ref_name], schema ) 
    else:
        new_param = schema['definitions'][ref_name]
        if 'name' in param:
            new_param['name'] = param['name']
            new_param['type'] = ref_name
        if 'in' in param:
            new_param['in'] = param['in']
        return parse_param( new_param, schema ) 


def get_params(schema, parameters):
  params = { 'query': list(), 'header': list(), 'body': list(), 'path': list() }
  for param in parameters:
     if 'in' in param and param['in'] == 'body':
         for k,v in param['schema']['properties'].items():
             v['name'] = k
             v['in'] = 'body'
             p = parse_param(v, schema)
             if p != None:
                 params[p['in']].append(p)
     else:
         p = parse_param(param, schema)
         if p != None:
           params[p['in']].append(p)
  return params

def parse_response_param(param, schema):
  if '$ref' not in param:
    result = dict()
    result['comment'] = get_comment(schema,param)
    result['type'] = get_param_type(schema, param)
    if result['type'] == 'array':
      if '$ref' in param['items']:
        ref_section, ref_name = parse_ref(param['items']['$ref'])
        result['type'] = "Vec<{}>".format(ref_name) 
      else:
        array_type = get_param_type(schema, param['items'])
        result['type'] = "Vec<{}>".format(array_type) 
    if 'name' in param:
      result['name'] = param['name'].replace('-','')
      result['snake'] = snake(param['name'].replace('-',''))
    return result 
  else:
    ref_section, ref_name = parse_ref(param['$ref'])
    p = schema[ref_section][ref_name]
    p['name'] = lcfirst(ref_name)
    p['snake'] = snake(ref_name)
    return parse_response_param(p, schema)

def get_response_params(schema, parameters):
  if '$ref' not in parameters:
    params = { 'headers': list(), 'schema': list() }
    for section_name, section in [('headers',parameters['headers']),('schema',parameters['schema']['properties'])]:
      for name, param in section.items():
        param['name'] = name.replace('-','')
        p = parse_response_param(param, schema)
        p['name'] = name.replace('-','')
        p['snake'] = snake(name.replace('-',''))
        if p != None:
          params[section_name].append(p)
  else:
    ref_name = parameters['$ref'].split("/")[-1]
    params = get_response_params( schema, schema['responses'][ref_name] ) 

  return params

def get_request(schema, url, method_name):
  result = dict()
  method = schema['paths'][url][method_name]
  result['url'] = url
  result['method'] = method_name 
  result['comment'] = get_comment(schema,method)
  result['operationId'] = method['operationId']
  result['name'] = ucfirst(method['operationId'])
  result['params'] = get_params(schema, method['parameters'])
  responses = list()
  for code, response in method['responses'].items():
#    if '$ref' not in response:
    if code == "200" or code == "201":
        item = {'name': result['name']+'Response',
                'code': 200,
                'comment': get_comment(schema,response),
                'params': get_response_params(schema, response)} 
        responses.append(item)
        break
  result['responses'] = responses
  return result 

def get_definition_enum(schema,name):
  result = dict() 
  enum = schema['definitions'][name]
  result['comment'] = get_comment(schema,enum)
  result['name'] = ucfirst(name.replace('-',''))
  result['camel'] = camel(name.replace('-',''))
  variants = list()
  for v in enum['enum']:
    v = v.replace('-','');
    variant = { 'name': v, 'camel': camel(v)}
    variants.append(variant) 
  result['variants'] = variants 
  return result 

def get_definition_struct(schema,name):
  result = dict();
  struct = schema['definitions'][name]
  result['comment'] = get_comment(schema, struct)
  result['name'] = name.replace('-','')
  result['camel'] = camel(name)
  params = list()
  for param_name, param_value in struct['properties'].items(): 
    t = parse_response_param(param_value, schema)
    t['name'] = param_name.replace('-','')
    t['snake'] = snake(param_name.replace('-',''))
    params.append(t)
  result['params'] = params
  return result

def run():
  args = get_args()
  schema = get_schema()
  file_loader = FileSystemLoader('./tools/tmpl')
  env = Environment(loader=file_loader)
  request_tmpl = env.get_template('request_builder.tmpl')
  enum_tmpl = env.get_template('enum.tmpl')
  struct_tmpl = env.get_template('struct.tmpl')
  if args.type == 'request':
    modules = list()
    for url,obj in schema['paths'].items():
      if url.startswith(args.url) and (args.method in obj or args.method == 'all'):
        if args.method == 'all':
            methods = list(schema['paths'][url].keys())
        else:
            methods = [args.method]
        for method in methods:
          if method == 'post' and url == '/accounts/{accountID}/orders':
            for variant in ['MarketOrder', 'LimitOrder', 'StopOrder']:
              r = get_request(schema, url, method)
              r['name'] = "Create{}".format(variant) 
              r['params']['body'][0]['type'] = variant
              r['params']['body'][0]['comment'] = "/// A request for a {} ".format(variant)
              r['operationId'] = "create{}".format(variant)
              #pprint(r)
              modules.append(snake(r['name']))
              print(request_tmpl.render(url=r['url'], method=r['method'], comment=r['comment'], operationId=r['operationId'], name=r['name'], snake=snake(r['name']), params=r['params'], responses=r['responses'])) 
          else:
            r = get_request(schema, url, method)
            modules.append(snake(r['name']))
            print(request_tmpl.render(url=r['url'], method=r['method'], comment=r['comment'], operationId=r['operationId'], name=r['name'], snake=snake(r['name']), params=r['params'], responses=r['responses'])) 
    for m in modules:
        print("pub use {}::*;".format(m))

  elif args.type == 'definition' and args.gen == 'enum':
    e = get_definition_enum(schema, args.url)
    print( enum_tmpl.render(name=e['name'], comment=e['comment'], camel=e['camel'], variants=e['variants']) )
  elif args.type == 'definition' and args.gen == 'struct':
    if args.url != 'all':
      e = get_definition_struct(schema, args.url)
      print (struct_tmpl.render(name=e['name'], comment=e['comment'], camel=e['camel'], params=e['params'])) 
    else:
      for name, obj in schema['definitions'].items():
        if 'enum' in obj:
          e = get_definition_enum(schema, name)
          print (num_tmpl.render(name=e['name'], comment=e['comment'], camel=e['camel'], variants=e['variants'])) 
        elif obj['type'] == 'object':
          e = get_definition_struct(schema, name)
          print( struct_tmpl.render(name=e['name'], comment=e['comment'], camel=e['camel'], params=e['params'])) 

if __name__ == '__main__':
  run()
