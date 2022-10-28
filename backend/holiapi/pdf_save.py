from typing import List
import fitz
import io
from PIL import Image
from holiapi.config import PATH


def save_imgs_from_pdf(path, hexdigest) -> List[str]:
    uploaded_pdf = fitz.open(path)
    img_exts = []
    picture_count = 0

    for page in uploaded_pdf:
        for img in page.get_images():
            # get the XREF of the image
            xref = img[0]
            
            # extract the image bytes
            base_image = uploaded_pdf.extract_image(xref)
            image_bytes = base_image["image"]

            # get the image extension
            image_ext = base_image["ext"]
            img_exts.append(image_ext)

            img = Image.open(io.BytesIO(image_bytes))

            picture_path = f"{hexdigest}_{picture_count}.{image_ext}"

            img.save(f"{PATH}/static/images/{picture_path}")
            picture_count+=1
    return img_exts