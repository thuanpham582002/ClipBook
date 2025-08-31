import Dexie, {Table} from 'dexie';

export enum ClipType {
  Text,
  Link,
  Email,
  Color,
  Image,
  File
}

export class LinkPreviewDetails {
  id?: number;
  url: string = "";
  title: string = "";
  description: string = "";
  imageFileName: string = "";
  faviconFileName: string = "";

  constructor(url: string, title: string, description: string, imageFileName: string, faviconFileName: string) {
    this.url = url
    this.title = title
    this.description = description
    this.imageFileName = imageFileName
    this.faviconFileName = faviconFileName
  }
}

export class Clip {
  id?: number;
  name: string = "";
  content: string = "";
  type: ClipType = ClipType.Text;
  sourceApp: string = "";
  favorite: boolean = false;
  tags?: number[] = [];
  copyTime: Date = new Date();
  numberOfCopies: number = 1;
  imageFileName: string = "";
  imageThumbFileName: string = "";
  imageWidth: number = 0;
  imageHeight: number = 0;
  imageSizeInBytes: number = 0;
  imageText: string = "";
  filePath: string = "";
  filePathFileName: string = "";
  filePathThumbFileName: string = "";
  fileSizeInBytes: number = 0;
  fileFolder: boolean = false;
  rtf: string = "";
  html: string = "";

  constructor(type: ClipType, content: string, sourceApp: string) {
    this.type = type;
    this.content = content;
    this.sourceApp = sourceApp;
  }
}

class AppDatabase extends Dexie {
  public history!: Table<Clip, number>;
  public linkPreviews!: Table<LinkPreviewDetails, number>;

  constructor() {
    super('ClipBookDB');
    this.version(1).stores({
      history: '++id, title, content, type, sourceApp, favorite, copyTime, numberOfCopies, imageFileName, imageThumbFileName, imageWidth, imageHeight, imageSizeInBytes, imageText, filePath, filePathFileName, filePathThumbFileName, fileSizeInBytes, fileFolder, rtf, html',
      linkPreviews: '++id, url, title, description, imageFileName, faviconFileName'
    });
  }
}

const db = new AppDatabase();

export async function getAllClips(): Promise<Clip[]> {
  return db.history.toArray()
}

export async function addClip(clip: Clip) {
  await db.history.add(clip)
}

export async function updateClip(id: number, clip: Partial<Clip>) {
  await db.history.update(id, clip)
}

export async function deleteClip(id: number) {
  await db.history.delete(id)
}

export async function deleteAllClips() {
  await db.history.clear()
}

export async function saveLinkPreviewDetails(details: LinkPreviewDetails) {
  await db.linkPreviews.where('url').equals(details.url).delete()
  await db.linkPreviews.add(details)
}

export async function deleteLinkPreviewDetails(url: string) {
  await db.linkPreviews.where('url').equals(url).delete()
}

export async function getLinkPreviewDetails(url: string): Promise<LinkPreviewDetails | undefined> {
  return db.linkPreviews.where('url').equals(url).first()
}

export function getImageText(item: Clip): string {
  return item && (item.imageText || "")
}

export function getImageFileName(item: Clip): string {
  return item && (item.imageFileName || "")
}

export function getFilePath(item: Clip): string {
  return item && (item.filePath || "")
}

export function getRTF(item: Clip): string {
  return item && (item.rtf || "")
}

export function getHTML(item: Clip): string {
  return item && (item.html || "")
}

// Backup and restore types
export interface BackupData {
  version: string;
  timestamp: string;
  clips: Clip[];
  linkPreviews: LinkPreviewDetails[];
}

// Export all data for backup
export async function exportAllData(): Promise<BackupData> {
  const clips = await getAllClips();
  const linkPreviews = await db.linkPreviews.toArray();
  
  return {
    version: "1.0.0",
    timestamp: new Date().toISOString(),
    clips,
    linkPreviews
  };
}

// Import data from backup
export async function importData(data: BackupData, options: { merge?: boolean } = {}): Promise<void> {
  try {
    // Validate backup data
    if (!data.clips || !Array.isArray(data.clips)) {
      throw new Error('Invalid backup data: clips array is required');
    }

    // Clear existing data if not merging
    if (!options.merge) {
      await db.history.clear();
      await db.linkPreviews.clear();
    }

    // Import clips
    if (data.clips.length > 0) {
      // Convert dates from strings back to Date objects
      const processedClips = data.clips.map(clip => ({
        ...clip,
        copyTime: new Date(clip.copyTime),
        id: options.merge ? undefined : clip.id // Let DB assign new IDs when merging
      }));

      await db.history.bulkAdd(processedClips);
    }

    // Import link previews
    if (data.linkPreviews && data.linkPreviews.length > 0) {
      const processedPreviews = data.linkPreviews.map(preview => ({
        ...preview,
        id: options.merge ? undefined : preview.id // Let DB assign new IDs when merging
      }));

      await db.linkPreviews.bulkAdd(processedPreviews);
    }

    console.log(`✅ Successfully imported ${data.clips.length} clips and ${data.linkPreviews?.length || 0} link previews`);
  } catch (error) {
    console.error('❌ Import failed:', error);
    throw error;
  }
}
