import {ObjectManager} from "../../pools/object-manager";
import { ColorGroup } from "../../utils/types";

export class CheckboxUpdatePatch {
    public id_chain?: number[];
    public size_x?: number;
    public size_y?: number;
    public transform?: number[];
    public checked?: boolean;
    public style?: { border: ColorGroup, color: ColorGroup, background: ColorGroup};
    objectManager: ObjectManager;

    constructor(objectManager: ObjectManager) {
        this.objectManager = objectManager;
    }

    fromPatch(jsonMessage: any) {
        this.id_chain = jsonMessage["id_chain"];
        this.size_x = jsonMessage["size_x"];
        this.size_y = jsonMessage["size_y"];
        this.transform = jsonMessage["transform"];
        this.checked = jsonMessage["checked"];
        let style_parts = jsonMessage["style"];
        if (style_parts != undefined) {
            this.style = {
                border: style_parts["border"],
                background: style_parts["background"],
                color: style_parts["color"]
            };
        }
    }

    cleanUp(){
        this.id_chain = [];
        this.size_x = 0;
        this.size_y = 0;
        this.transform = [];
        this.checked = undefined;
        this.style = undefined;
    }
}