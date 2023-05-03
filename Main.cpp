/*
** EPITECH PROJECT, 2023
** B-OOP-400-PAR-4-1-raytracer-vincent.shao
** File description:
** Main
*/

#include "libconfig.h++"
#include <iostream>

void write_camera(libconfig::Config &cfg)
{

    libconfig::Setting &camera = cfg.getRoot().add("camera", libconfig::Setting::TypeGroup);

    libconfig::Setting &resolution = camera.add("resolution", libconfig::Setting::TypeGroup);
    resolution.add("width", libconfig::Setting::TypeInt) = 50;
    resolution.add("height", libconfig::Setting::TypeInt) = 1080;

    libconfig::Setting &position = camera.add("position", libconfig::Setting::TypeGroup);
    position.add("x", libconfig::Setting::TypeInt) = 1;
    position.add("y", libconfig::Setting::TypeInt) = 2;
    position.add("z", libconfig::Setting::TypeInt) = 3;

    libconfig::Setting &rotation = camera.add("rotation", libconfig::Setting::TypeGroup);
    rotation.add("x", libconfig::Setting::TypeInt) = 1;
    rotation.add("y", libconfig::Setting::TypeInt) = 2;
    rotation.add("z", libconfig::Setting::TypeInt) = 3;

    camera.add("fieldOfView", libconfig::Setting::TypeFloat) = 72.0;
}

void write_sphere(libconfig::Setting &primitive)
{
    libconfig::Setting &sphere = primitive.add("spheres", libconfig::Setting::TypeList);

    for (int i = 0; i != 2; i++) {
        auto &tmp_sphere = sphere.add(libconfig::Setting::TypeGroup);
        tmp_sphere.add("x", libconfig::Setting::TypeInt) = 0;
        tmp_sphere.add("y", libconfig::Setting::TypeInt) = 0;
        tmp_sphere.add("z", libconfig::Setting::TypeInt) = 0;
        auto &tmp_sphere_color = tmp_sphere.add("color", libconfig::Setting::TypeGroup);
        tmp_sphere_color.add("r", libconfig::Setting::TypeInt) = 0;
        tmp_sphere_color.add("g", libconfig::Setting::TypeInt) = 0;
        tmp_sphere_color.add("b", libconfig::Setting::TypeInt) = 0;
    }
}

void write_planes(libconfig::Setting &primitive)
{
    libconfig::Setting &planes = primitive.add("planes", libconfig::Setting::TypeList);
    planes.setFormat(libconfig::Setting::Format::FormatHex);
    for (int i = 0; i != 2; i++)
    {
        auto &tmp_planes = planes.add(libconfig::Setting::TypeGroup);
        tmp_planes.setFormat(libconfig::Setting::Format::FormatDefault);
        tmp_planes.add("axis", libconfig::Setting::TypeString) = "Z";
        tmp_planes.add("position", libconfig::Setting::TypeInt) = -20;
        auto &tmp_planes_color = tmp_planes.add("color", libconfig::Setting::TypeGroup);
        tmp_planes_color.add("r", libconfig::Setting::TypeInt) = 0;
        tmp_planes_color.add("g", libconfig::Setting::TypeInt) = 0;
        tmp_planes_color.add("b", libconfig::Setting::TypeInt) = 0;
    }
}

void write_primitive(libconfig::Config &cfg)
{
    libconfig::Setting &primitive = cfg.getRoot().add("primitives", libconfig::Setting::TypeGroup);

    write_planes(primitive);
    write_sphere(primitive);
}

void write_point(libconfig::Setting & light)
{
    libconfig::Setting &point = light.add("point", libconfig::Setting::TypeList);

    for (int i = 0; i != 2; i++) {
        auto &single_point = point.add(libconfig::Setting::TypeGroup);
        single_point.add("x", libconfig::Setting::TypeInt) = 0;
        single_point.add("y", libconfig::Setting::TypeInt) = 0;
        single_point.add("z", libconfig::Setting::TypeInt) = 0;
    }
}

void write_directional(libconfig::Setting & light)
{
    libconfig::Setting &directionnal = light.add("directional", libconfig::Setting::TypeList);
    for (int i = 0; i != 0; i++) {
        auto &single_directional = directionnal.add(libconfig::Setting::TypeGroup);
        single_directional.add("x", libconfig::Setting::TypeInt) = 0;
        single_directional.add("y", libconfig::Setting::TypeInt) = 0;
        single_directional.add("z", libconfig::Setting::TypeInt) = 0;
    }
}

void write_lights(libconfig::Config &cfg)
{
    libconfig::Setting &light = cfg.getRoot().add("lights", libconfig::Setting::TypeGroup);

    light.add("ambient", libconfig::Setting::TypeFloat) = 0.4;
    light.add("diffuse", libconfig::Setting::TypeFloat) = 0.6;
    write_point(light);
    write_directional(light);
}

void write_file()
{
    libconfig::Config cfg;
    cfg.setOption(libconfig::Config::OptionColonAssignmentForGroups, false);
    cfg.setOption(libconfig::Config::OptionFsync, true);
    cfg.setOption(libconfig::Config::OptionOpenBraceOnSeparateLine, false);
    cfg.setOption(libconfig::Config::OptionOpenBraceOnSeparateLine, false);
    write_camera(cfg);
    write_primitive(cfg);
    write_lights(cfg);
    cfg.writeFile("Lol.cfg");
}

int read_file()
{
 try {
        libconfig::Config cfg;
        cfg.readFile("Lol.cfg");

        // Get camera settings
        libconfig::Setting& camera = cfg.lookup("camera");
        int width = camera["resolution"]["width"];
        int height = camera["resolution"]["height"];
        int x = camera["position"]["x"];
        int y = camera["position"]["y"];
        int z = camera["position"]["z"];
        float fov = camera["fieldOfView"];

        // Get sphere settings
        libconfig::Setting& spheres = cfg.lookup("primitives.spheres");
        for (int i = 0; i < spheres.getLength(); i++) {
            int x = spheres[i]["x"];
            int y = spheres[i]["y"];
            int z = spheres[i]["z"];
            int color_r = spheres[i]["color"]["r"];
            int color_g = spheres[i]["color"]["g"];
            int color_b = spheres[i]["color"]["b"];
            std::cout << "sphere:" << i << " x:" << x << " y:" << y << " z:" << z << " RGB: " << color_r << color_g << color_b << std::endl;
            // do something with the sphere settings...
        }

        // Get plane settings
        libconfig::Setting& planes = cfg.lookup("primitives.planes");
        for (int i = 0; i < planes.getLength(); i++) {
            std::string axis = planes[i]["axis"];
            int position = planes[i]["position"];
            int color_r = planes[i]["color"]["r"];
            int color_g = planes[i]["color"]["g"];
            int color_b = planes[i]["color"]["b"];
            std::cout << "planes:" << i << " Axes:" << axis << " RGB: " << color_r << color_g << color_b << std::endl;
            // do something with the plane settings...
        }

        // Get light settings
        double ambient = cfg.lookup("lights.ambient");
        double diffuse = cfg.lookup("lights.diffuse");
        std::cout << "ligth:Ambient: " << ambient << std::endl;
        std::cout << "ligth:diffuse: " << diffuse << std::endl;
        libconfig::Setting &point_lights = cfg.lookup("lights.point");
        for (int i = 0; i < point_lights.getLength(); i++) {
            int x = point_lights[i]["x"];
            int y = point_lights[i]["y"];
            int z = point_lights[i]["z"];
            std::cout << "light:" << i <<  " x:" << x << " y:" << y << " z:" << z << std::endl;
            // do something with the point light settings...
        }
        libconfig::Setting& directional_lights = cfg.lookup("lights.directional");
        for (int i = 0; i < directional_lights.getLength(); i++) {
            // do something with the directional light settings...
        }
    } catch(const libconfig::FileIOException &fioex) {
        std::cerr << "I/O error while reading file." << std::endl;
        return(EXIT_FAILURE);
    } catch(const libconfig::ParseException &pex) {
        std::cerr << "Parse error at " << pex.getFile() << ":" << pex.getLine()
                  << " - " << pex.getError() << std::endl;
        return(EXIT_FAILURE);
    } catch(const libconfig::SettingNotFoundException &nfex) {
        std::cerr << "Error: setting not found." << std::endl;
        return(EXIT_FAILURE);
    }
    return 0;
}

int main(int ac, char **av)
{
    write_file();
    int a  = read_file();
    std::cout << a << std::endl;
}